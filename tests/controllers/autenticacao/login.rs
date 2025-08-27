use actix_web::http::StatusCode;
use actix_web::http::header::REFERER;
use actix_web::test::TestRequest;
use actix_web::web::Data;
use dominio::autenticacao::HasherDeSenha;
use dominio::test::fabricas_de_entidades::usuario_modelo::UsuarioModeloParcial;
use inertia_rust::Inertia;
use inertia_rust::test::{InertiaTestRequest, IntoAssertableInertia};
use pretty_assertions::assert_eq;
use projetario::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;
use rstest::rstest;
use serde_json::json;
use sqlx::PgPool;

use crate::common::fixtures::db_guard::{DBGuard, db_guard};
use crate::common::fixtures::inertia::inertia;
use crate::common::setup::__setup;
use crate::common::utils::headers::{extraia_cookie_da_sessao, extraia_valor_do_header_location};
use crate::instanciar_app;

#[rstest]
#[awt]
#[tokio::test]
pub async fn um_usuario_deveria_poder_se_autenticar(
    __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] db_guard: DBGuard<'_>,
) {
    inserir_usuario_no_db(db_guard.as_ref()).await;

    let app = instanciar_app!(inertia, db_guard.clone_a_conexao());

    let resposta = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "registro_aluno": "a256020",
            "senha": "123456"
        }))
        .inertia()
        .insert_header((REFERER, "/"))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::SEE_OTHER, resposta.status());

    let local_do_redirect = extraia_valor_do_header_location(&resposta);

    assert_eq!(
        "/", local_do_redirect,
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
}

#[rstest]
#[awt]
#[tokio::test]
pub async fn soh_usuarios_nao_autenticados_deveriam_poder_ver_a_pagina_de_login(
    __setup: (),
    #[future] inertia: Data<Inertia>,
    #[future] db_guard: DBGuard<'_>,
) {
    inserir_usuario_no_db(db_guard.as_ref()).await;

    let app = instanciar_app!(inertia, db_guard.clone_a_conexao());

    let resposta_login = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "registro_aluno": "a256020",
            "senha": "123456"
        }))
        .inertia()
        .insert_header((REFERER, "/projetos"))
        .send_request(&app)
        .await;

    assert_eq!(StatusCode::SEE_OTHER, resposta_login.status());

    let tentativa_de_ir_para_login = TestRequest::post()
        .uri("/autenticacao/login")
        .set_json(json!({
            "registro_aluno": "a256020",
            "senha": "123456"
        }))
        .inertia()
        .cookie(extraia_cookie_da_sessao(&resposta_login))
        .send_request(&app)
        .await;

    assert_eq!(
        StatusCode::UNAUTHORIZED,
        tentativa_de_ir_para_login.status()
    );
}

async fn inserir_usuario_no_db(db_conn: &PgPool) {
    let mut usuario = UsuarioModeloParcial::aluno();
    usuario.registro_aluno = Some("a256020".into());
    usuario.periodo = Some(2);
    usuario.senha_hash = Some(
        ComparadorEHasherDeSenhaCrypto::novo()
            .aplique_hash("123456")
            .unwrap(),
    );

    let usuario = usuario.into_entidade();

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
    .bind(usuario.periodo.to_optional::<i16>())
    .execute(db_conn)
    .await
    .expect("Não foi possível inserir o aluno mockado no banco de dados");
}
