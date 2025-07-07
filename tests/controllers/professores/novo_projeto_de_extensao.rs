use actix_web::http::StatusCode;
use actix_web::test::{TestRequest, init_service};
use actix_web::web::Data;
use db_seeder::usuarios::inserir_usuarios;
use inertia_rust::Inertia;
use inertia_rust::test::{InertiaTestRequest, IntoAssertableInertia};
use pretty_assertions::assert_eq;
use projetario::dominio::identidade::entidades::professor::Professor;
use projetario::dominio::projetos::entidades::projeto::Projeto;
use projetario::libs::actix::server::get_server;
use rstest::rstest;
use serde_json::json;
use sqlx::query_as;

use crate::common::fixtures::db_guard::{DBGuard, db_guard};
use crate::common::fixtures::inertia::inertia;
use crate::common::setup::__setup;
use crate::common::utils::headers::extraia_cookie_da_sessao;

#[rstest]
#[case("reginaldo@utfpr.com", "12345", "Reginaldo Ré")] // um Professor
#[case("cremoso@utfpr.com", "12345", "Paulo Sabo")] // um Administrador
#[awt]
#[tokio::test]
pub async fn um_professor_deveria_poder_criar_um_projeto_de_extensao(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
    #[case] email_professor: String,
    #[case] senha_professor: String,
    #[case] nome_professor: String,
) {
    inserir_usuarios(db_guard.as_ref()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({"email": email_professor, "senha": senha_professor}))
        .inertia()
        .send_request(&app)
        .await;

    let cookies = extraia_cookie_da_sessao(&autenticacao);

    // region: --- Garante que não houve erros
    let _resposta = TestRequest::post()
        .uri("/professores/projetos/extensao/criar_e_associar")
        .inertia()
        .cookie(cookies.clone())
        .set_json(json!({
            "titulo": "Projeto Foo",
            "descricao": "<h1>Titulo do Projeto</h1><p>Corpo do projeto</p><p>Com no mínimo 50 caracteres</p>",
        }))
        .send_request(&app)
        .await;

    let pagina = TestRequest::get()
        .uri("/dev/hello/world")
        .cookie(cookies.clone())
        .inertia()
        .send_request(&app)
        .await
        .into_assertable_inertia();

    assert!(pagina.get_props()["errors"].as_object().unwrap().is_empty());
    // endregion: --- Garante que não houve erros

    // region: --- Garante que o projeto foi persistido no banco de dados
    let (qtd_projetos,): (i64,) = query_as("SELECT COUNT(id) FROM projeto")
        .fetch_one(db_guard.as_ref())
        .await
        .unwrap();

    assert_eq!(1, qtd_projetos);

    let projeto: Projeto = query_as(
        "SELECT id, titulo, descricao, tipo, registrado_em, iniciado_em, atualizado_em, cancelado_em, concluido_em \
        FROM projeto"
    ).fetch_one(db_guard.as_ref()).await.unwrap();

    // endregion --- Garante que o projeto foi persistido no banco de dados

    // region: --- Garante que o projeto foi associado com o professor autenticado na requisição
    let coordenador_projeto: Professor = query_as(
        "SELECT * FROM usuario WHERE usuario.id = (SELECT id_coordenador FROM coordenador_projeto WHERE id_projeto = $1)",
    )
    .bind(projeto.obtenha_id())
    .fetch_one(db_guard.as_ref()).await.unwrap();

    assert_eq!(
        nome_professor,
        coordenador_projeto.obtenha_usuario().obtenha_nome()
    );
    // endregion: --- Garante que o projeto foi associado com o professor autenticado na requisição
}

#[rstest]
#[awt]
#[tokio::test]
async fn um_aluno_nao_deveria_poder_criar_um_projeto_de_extensao(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
) {
    inserir_usuarios(db_guard.as_ref()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({"registro_aluno": "a2250331", "senha": "12345"}))
        .inertia()
        .send_request(&app)
        .await;

    let cookies = extraia_cookie_da_sessao(&autenticacao);

    let resposta = TestRequest::post()
        .uri("/professores/projetos/extensao/criar_e_associar")
        .inertia()
        .cookie(cookies.clone())
        .set_json(json!({
            "titulo": "Projeto Foo",
            "descricao": "<h1>Titulo do Projeto</h1><p>Corpo do projeto</p><p>Com no mínimo 50 caracteres</p>",
        }))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::UNAUTHORIZED, resposta.status());
}

#[rstest]
#[awt]
#[tokio::test]
async fn deveria_criar_projeto_com_data_de_inicio_customizada(
    __setup: (),
    #[future] db_guard: DBGuard<'_>,
    #[future] inertia: Data<Inertia>,
) {
    inserir_usuarios(db_guard.as_ref()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let autenticacao = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({"email": "reginaldo@utfpr.com", "senha": "12345"}))
        .inertia()
        .send_request(&app)
        .await;

    let cookies = extraia_cookie_da_sessao(&autenticacao);

    let _resposta = TestRequest::post()
        .uri("/professores/projetos/extensao/criar_e_associar")
        .inertia()
        .cookie(cookies.clone())
        .set_json(json!({
            "titulo": "Projeto Foo",
            "descricao": "<h1>Titulo do Projeto</h1><p>Corpo do projeto</p><p>Com no mínimo 50 caracteres</p>",
            "data_de_inicio": "2024-07-07"
        }))
        .send_request(&app)
        .await;

    let pagina = TestRequest::get()
        .uri("/dev/hello/world")
        .cookie(cookies.clone())
        .inertia()
        .send_request(&app)
        .await
        .into_assertable_inertia();

    assert!(pagina.get_props()["errors"].as_object().unwrap().is_empty());

    let projeto: Projeto = query_as("SELECT * FROM projeto")
        .fetch_one(db_guard.as_ref())
        .await
        .unwrap();

    assert_eq!("2024-07-07", projeto.obtenha_data_de_inicio().to_string(),);
}
