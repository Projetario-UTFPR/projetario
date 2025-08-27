use actix_session::SessionMiddleware;
use actix_web::App;
use actix_web::body::{BoxBody, EitherBody};
use actix_web::cookie::{Key, SameSite};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::NormalizePath;
use config::app::{AppConfig, RustEnv};
use inertia_rust::InertiaService;
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
        .wrap(NormalizePath::trim())
        .inertia_route("/", "index")
        .inertia_route("/dev/hello/world", "hello-world")
        .configure(WebRouter::register)
        .service(actix_files::Files::new("/", "./public/").prefer_utf8(true))
}
