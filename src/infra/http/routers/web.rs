use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::ProfessoresRouter;

pub struct WebRouter;

impl RouterRegistrable for WebRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.configure(ControllerAutenticacao::register)
            .configure(ProfessoresRouter::register);
    }
}
