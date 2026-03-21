use actix_session::SessionMiddleware;
use actix_web::body::BoxBody;
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::NormalizePath;
use actix_web::{App, web};
use config::app::{AppConfig, RustEnv};
use inertia_sessions::file_session::FileSessionStore;
use inertia_sessions::middlewares::garbage_collector::GarbageCollectorMiddleware;
use inertia_sessions::middlewares::reflash_temporary_session::ReflashTemporarySessionMiddleware;

use crate::infra::http::RouterRegistrable;
use crate::infra::http::middlewares::usuario_da_requisicao::MiddlewareUsuarioDaRequisicao;
use crate::infra::http::routers::web::WebRouter;
use crate::libs::inertia::middleware::get_inertia_middleware;

pub fn get_server() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app_config = AppConfig::get();
    let key = Key::from(app_config.app_key);
    let storage = FileSessionStore::default();

    App::new()
        .wrap(NormalizePath::trim())
        .service(actix_files::Files::new("/bundle/", "./public/bundle/").prefer_utf8(true))
        .service(actix_files::Files::new("/tiny-mce/", "./public/tiny-mce/").prefer_utf8(true))
        .route(
            "/favicon.ico",
            web::get()
                .to(|| async { actix_files::NamedFile::open_async("./public/favicon.ico").await }),
        )
        .service(
            web::scope("")
                .wrap(GarbageCollectorMiddleware)
                .wrap(get_inertia_middleware())
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
                .configure(WebRouter::register)
                // serviço de fallback de arquivos o ideal é manter tudo fora desse escopo, mas se ainda não tiver
                // sido montado, esse aqui garante que vai ser entregue mesmo que sofra modificações decorrentes
                // dos middlewares.
                .service(actix_files::Files::new("/", "./public/").prefer_utf8(true)),
        )
}
