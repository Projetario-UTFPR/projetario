use std::sync::Arc;

use actix_session::{SessionExt, SessionMiddleware};
use actix_web::body::{BoxBody, EitherBody};
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::NormalizePath;
use actix_web::{App, HttpMessage, web};
use config::app::{AppConfig, RustEnv};
use futures_util::FutureExt;
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::{InertiaProp, InertiaService, IntoInertiaPropResult, hashmap};
use inertia_sessions::file_session::FileSessionStore;
use inertia_sessions::middlewares::garbage_collector::GarbageCollectorMiddleware;
use inertia_sessions::middlewares::reflash_temporary_session::ReflashTemporarySessionMiddleware;
use serde_json::Map;

use crate::dominio::identidade::traits::IntoUsuarioModelo;
use crate::infra::http::controllers::Controller;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::projetos_de_extensao::ControllerProjetosDeExtensao;
use crate::infra::http::middlewares::usuario_da_requisicao::{
    MiddlewareUsuarioDaRequisicao,
    UsuarioDaRequisicao,
};
use crate::infra::http::presenters::usuario_modelo::UsuarioModeloPresenter;

pub fn get_server() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<EitherBody<BoxBody>>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app_config = AppConfig::get();
    let key = Key::from(app_config.app_key);
    let storage = FileSessionStore::default();

    App::new()
        .wrap(GarbageCollectorMiddleware)
        .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
            let usuario = req.extensions().get::<UsuarioDaRequisicao>().unwrap_or_else(|| {
                log::warn!("Usuário da requisição não encontrada pelo `InertiaMiddleware`, caindo para o usuário convidado.");
                &UsuarioDaRequisicao::Convidado
            }).clone();

            let usuario = match usuario {
                UsuarioDaRequisicao::Convidado => None,
                UsuarioDaRequisicao::Aluno(aluno) => Some(aluno.into_usuario_modelo()),
                UsuarioDaRequisicao::Professor(professor) => Some(professor.into_usuario_modelo())
            };

            let autenticacao = usuario.map(|usuario| {
                hashmap!["usuario".to_string() => UsuarioModeloPresenter::apresente(&usuario)]
            });

            let flash = req
                .get_session()
                .remove(app_config.sessions_flash_key)
                .map(|flash_map| serde_json::from_str::<Map<_, _>>(&flash_map).unwrap_or_default())
                .unwrap_or_default();

            async { hashmap![
                "flash" => InertiaProp::always(flash),
                "autenticacao" => InertiaProp::Data(autenticacao.into_inertia_value()) 
            ] }.boxed_local()
        })))
        .wrap(ReflashTemporarySessionMiddleware)
        .wrap(MiddlewareUsuarioDaRequisicao)
        .wrap(
            SessionMiddleware::builder(storage, key)
                .cookie_domain(None)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .cookie_secure(app_config.environment == RustEnv::Production)
                .cookie_name(app_config.sessions_cookie_name.to_string())
                .build(),
        )
        .wrap(NormalizePath::trim())
        .inertia_route("/", "index")
        .inertia_route("/dev/hello/world", "hello-world")
        .configure(ControllerAutenticacao::register)
        .service(web::scope("/professores").configure(ControllerProjetosDeExtensao::register))
}
