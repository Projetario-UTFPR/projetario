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
use inertia_rust::{InertiaProp, hashmap};
use inertia_sessions::file_session::FileSessionStore;
use inertia_sessions::middlewares::garbage_collector::GarbageCollectorMiddleware;
use inertia_sessions::middlewares::reflash_temporary_session::ReflashTemporarySessionMiddleware;
use serde_json::Map;

use crate::infra::http::controllers::Controller;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::projetos_de_extensao::ControllerProjetosDeExtensao;
use crate::infra::http::middlewares::usuario_da_requisicao::{
    MiddlewareUsuarioDaRequisicao,
    UsuarioDaRequisicao,
};

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
            let flash = req
                .get_session()
                .remove(app_config.sessions_flash_key)
                .map(|flash_map| serde_json::from_str::<Map<_, _>>(&flash_map).unwrap_or_default())
                .unwrap_or_default();

            async { hashmap![ "flash" => InertiaProp::always(flash) ] }.boxed_local()
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
        .configure(ControllerAutenticacao::register)
        .service(web::scope("/professores").configure(ControllerProjetosDeExtensao::register))
}
