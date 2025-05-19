use actix_web::web::ServiceConfig;

pub mod web;

pub trait RouterTrait {
    fn register(cfg: &mut ServiceConfig);
}
