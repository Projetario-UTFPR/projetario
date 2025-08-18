use actix_web::web::ServiceConfig;

pub mod controllers;
pub mod middlewares;
pub mod presenters;
pub mod resposta_incerta_do_servidor;
pub mod routers;

pub trait RouterRegistrable {
    fn register(cfg: &mut ServiceConfig);
}
