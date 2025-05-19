use actix_session::{SessionExt, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    App,
};
use config::app::{AppConfig, RustEnv};
use futures_util::FutureExt;
use inertia_rust::{actix::InertiaMiddleware, hashmap, InertiaProp};
use inertia_sessions::{
    file_session::FileSessionStore,
    middlewares::{
        garbage_collector::GarbageCollectorMiddleware,
        reflash_temporary_session::ReflashTemporarySessionMiddleware,
    },
};
use serde_json::Map;
use std::sync::Arc;

pub fn get_server() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app_config = AppConfig::get();
    let key = Key::from(app_config.app_key);
    let storage = FileSessionStore::default();

    App::new()
        .wrap(InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
            let flash = req
                .get_session()
                .remove(app_config.sessions_flash_key)
                .map(|flash_map| serde_json::from_str::<Map<_, _>>(&flash_map).unwrap_or_default())
                .unwrap_or_default();

            async { hashmap![ "flash" => InertiaProp::always(flash) ] }.boxed_local()
        })))
        .wrap(ReflashTemporarySessionMiddleware)
        .wrap(GarbageCollectorMiddleware)
        .wrap(
            SessionMiddleware::builder(storage, key)
                .cookie_domain(None)
                .cookie_http_only(true)
                .cookie_same_site(SameSite::Lax)
                .cookie_secure(app_config.environment == RustEnv::Production)
                .build(),
        )
}
