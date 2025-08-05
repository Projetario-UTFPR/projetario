use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, init_service};
use actix_web::web::Data;
use inertia_rust::Inertia;
use inertia_rust::test::{InertiaTestRequest, IntoAssertableInertia};
use pretty_assertions::assert_eq;
use projetario::dominio::identidade::traits::IntoUsuarioModelo;
use projetario::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use projetario::libs::actix::server::get_server;
use projetario::utils::test::fabricas_de_entidades::aluno::AlunoParcial;
use projetario::utils::test::fabricas_de_entidades::professor::ProfessorParcial;
use projetario::utils::test::fabricas_de_entidades::projeto::ProjetoParcial;
use rstest::rstest;
use serde_json::{Value, json};

use crate::common::fixtures::db_guard::{DBGuard, db_guard};
use crate::common::fixtures::inertia::inertia;
use crate::common::setup::__setup;
use crate::common::utils::headers::extraia_cookie_da_sessao;
use crate::common::utils::queries::{criar_projeto_e_associar, salvar_usuario};

#[rstest]
#[case(json!({}))]
#[case(json!({
    "id_projeto": "5960bfc4-d29a-44db-b396-c963b2aac193", "horas_por_semana": 10, "imagem": "url inválido",
    "quantidade": 1, "link_edital": "https://www.foo.com", "conteudo": "conteudo", "titulo": "minimo 5",
    "inscricoes_ate": "2025-07-15T03:00:06.223Z"
}))]
#[case(json!({
    "id_projeto": "5960bfc4-d29a-44db-b396-c963b2aac193", "horas_por_semana": 10, "imagem": "https://www.foo.com",
    "quantidade": 1, "link_edital": "foo", "conteudo": "conteudo", "titulo": "minimo 5",
    "inscricoes_ate": "2025-07-15T03:00:06.223Z"
}))]
#[case(json!({
    "id_projeto": "5960bfc4-d29a-44db-b396-c963b2aac193", "horas_por_semana": 10, "imagem": "https://www.foo.com",
    "quantidade": 1, "link_edital": "https://www.foo.com", "conteudo": "conteudo", "titulo": "foo",
    "inscricoes_ate": "2025-07-15T03:00:06.223Z"
}))]
#[case(json!({
    "id_projeto": "5960bfc4-d29a-44db-b396-c963b2aac193", "horas_por_semana": 10, "imagem": "https://www.foo.com",
    "quantidade": 1, "link_edital": "https://www.foo.com", "conteudo": "conteudo", "titulo": "minimo 5",
    "inscricoes_ate": "2025-07-15T03:00:06.223Z"
}))]
#[awt]
#[tokio::test]
async fn ninguem_deveria_poder_criar_vagas_invalidas(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
    #[case] body: Value,
) {
    let professor = ProfessorParcial::default().into_entidade();
    salvar_usuario(db_guard.as_ref(), &professor.clone().into_usuario_modelo()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "email": professor.obtenha_usuario().obtenha_email(),
            "senha": professor.obtenha_usuario().obtenha_hash_da_senha().unwrap()
        }))
        .inertia()
        .send_request(&app)
        .await;

    let cookie = extraia_cookie_da_sessao(&autenticacao);

    let resposta = TestRequest::post()
        .uri("/professores/vagas/criar")
        .cookie(cookie)
        .inertia()
        .set_json(&body)
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::FOUND, resposta.status());

    let count: i64 = sqlx::query_scalar("SELECT COUNT(id) FROM vaga")
        .fetch_one(db_guard.as_ref())
        .await
        .unwrap();

    assert_eq!(0, count);
}

#[rstest]
#[awt]
#[tokio::test]
async fn um_aluno_nao_deveria_poder_criar_vagas(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
) {
    let aluno = AlunoParcial::default().into_entidade();
    salvar_usuario(db_guard.as_ref(), &aluno.clone().into_usuario_modelo()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "registro_aluno": aluno.obtenha_registro_de_aluno(),
            "senha": aluno.obtenha_usuario().obtenha_hash_da_senha().unwrap()
        }))
        .inertia()
        .send_request(&app)
        .await;

    let cookie = extraia_cookie_da_sessao(&autenticacao);

    let resposta = TestRequest::post()
        .uri("/professores/vagas/criar")
        .cookie(cookie.clone())
        .inertia()
        .set_json(json!({}))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::UNAUTHORIZED, resposta.status());
}

#[rstest]
#[awt]
#[tokio::test]
async fn um_professor_deveria_poder_criar_vagas_para_seus_projetos(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
) {
    let professor = ProfessorParcial::default().into_entidade();
    salvar_usuario(db_guard.as_ref(), &professor.clone().into_usuario_modelo()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "email": professor.obtenha_usuario().obtenha_email(),
            "senha": professor.obtenha_usuario().obtenha_hash_da_senha().unwrap()
        }))
        .inertia()
        .send_request(&app)
        .await;

    let cookie = extraia_cookie_da_sessao(&autenticacao);

    let projeto = ProjetoParcial::default().into_entidade();
    criar_projeto_e_associar(
        db_guard.as_ref(),
        &projeto,
        &professor,
        TipoDeCoordenacao::Coordenador,
    )
    .await;

    let resposta = TestRequest::post()
        .uri("/professores/vagas/criar")
        .cookie(cookie.clone())
        .inertia()
        .set_json(json!({
            "id_projeto": projeto.obtenha_id(),
            "horas_por_semana": 10,
            "imagem": "https://www.foo.com",
            "quantidade": 2,
            "link_edital": "https://www.foo.com",
            "conteudo": "conteudo",
            "inscricoes_ate": "2500-07-15T03:00:06.223Z"
        }))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::FOUND, resposta.status());

    let page = TestRequest::get()
        .uri("/dev/hello/world")
        .inertia()
        .cookie(cookie)
        .send_request(&app)
        .await
        .into_assertable_inertia();

    dbg!(&page);
    assert!(page.get_props()["errors"].as_object().unwrap().is_empty());

    let count: i64 = sqlx::query_scalar("SELECT COUNT(id) FROM vaga")
        .fetch_one(db_guard.as_ref())
        .await
        .unwrap();

    assert_eq!(1, count);
}
