use actix_web::web;

use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::acoes::tema::TemaController;

pub mod tema;

pub struct AcoesControllersGroup;

impl RouterRegistrable for AcoesControllersGroup {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::scope("acoes").configure(TemaController::register));
    }
}
