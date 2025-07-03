use actix_web::http::StatusCode;
use actix_web::http::header::REFERER;
use actix_web::test::{TestRequest, init_service};
use actix_web::web::Data;
use inertia_rust::Inertia;
use inertia_rust::test::{InertiaTestRequest, IntoAssertableInertia};
use pretty_assertions::assert_eq;
use projetario::dominio::autenticacao::HasherDeSenha;
use projetario::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;
use projetario::libs::actix::server::get_server;
use projetario::utils::test::fabricas_de_entidades::usuario_modelo::{
    FabricaUsuarioModelo,
    UsuarioModeloConstrutor,
};
use rstest::rstest;
use serde_json::json;
use sqlx::PgPool;

use crate::common::fixtures::db_guard::{DBGuard, db_guard};
use crate::common::fixtures::inertia::inertia;
use crate::common::setup::__setup;
use crate::common::utils::headers::{extraia_cookie_da_sessao, extraia_valor_do_header_location};

#[rstest]
#[awt]
#[tokio::test]
pub async fn um_usuario_deveria_poder_se_autenticar(
    __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] db_guard: DBGuard<'_>,
) {
    inserir_usuario_no_db(db_guard.as_ref()).await;

    let app = init_service(
        get_server()
            .app_data(inertia)
            .app_data(db_guard.clone_a_conexao()),
    )
    .await;

    let resposta = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "registro_aluno": "a256020",
            "senha": "123456"
        }))
        .inertia()
        .insert_header((REFERER, "/dev/hello/world"))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::FOUND, resposta.status());

    let local_do_redirect = extraia_valor_do_header_location(&resposta);

    assert_eq!(
        "/dev/hello/world", local_do_redirect,
        "Deveria ter autenticado e mandado de volta para a página de origem."
    );

    let resposta_subsequente = TestRequest::get()
        .uri(local_do_redirect)
        .inertia()
        .cookie(extraia_cookie_da_sessao(&resposta))
        .send_request(&app)
        .await;

    assert!(resposta_subsequente.status().is_success());

    let pagina = resposta_subsequente.into_assertable_inertia();

    assert!(pagina.get_props()["autenticacao"].get("usuario").is_some());
    assert!(
        pagina.get_props()["flash"]["sucessoLogin"]
            .to_string()
            .to_lowercase()
            .contains("autenticado com sucesso"),
        "Deveria ter redirecionado com um flash message de sucesso no login."
    );

    assert!(pagina.get_props().get("usuario").is_some());
}

async fn inserir_usuario_no_db(db_conn: &PgPool) {
    let mut usuario = UsuarioModeloConstrutor::aluno();
    usuario.registro_aluno = Some("a256020".into());
    usuario.periodo = Some(2);
    usuario.senha_hash = Some(
        ComparadorEHasherDeSenhaCrypto::novo()
            .aplique_hash("123456")
            .unwrap(),
    );

    let usuario = FabricaUsuarioModelo::obtenha_entidade(usuario);

    sqlx::query(
        "INSERT INTO \"usuario\" ( \
            id, \
            nome, \
            email, \
            senha_hash, \
            url_curriculo_lattes, \
            cargo, \
            registrado_em, \
            atualizado_em, \
            desativado_em, \
            registro_aluno, \
            periodo \
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
    )
    .bind(usuario.id)
    .bind(usuario.nome)
    .bind(usuario.email)
    .bind(usuario.senha_hash)
    .bind(usuario.url_curriculo_lattes)
    .bind(usuario.cargo)
    .bind(usuario.registrado_em)
    .bind(usuario.atualizado_em)
    .bind(usuario.desativado_em)
    .bind(usuario.registro_aluno)
    .bind(usuario.periodo)
    .execute(db_conn)
    .await
    .expect("Não foi possível inserir o aluno mockado no banco de dados");
}
