use actix_web::web::ServiceConfig;

pub mod controllers;
pub mod middlewares;
pub mod presenters;
pub mod routers;

pub trait RouterRegistrable {
    fn register(cfg: &mut ServiceConfig);
}
