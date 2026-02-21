use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::acoes::AcoesControllersGroup;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::ProfessoresControllersGroup;
use crate::infra::http::controllers::vagas::ControllerVagas;

pub struct WebRouter;

impl RouterRegistrable for WebRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.configure(ControllerAutenticacao::register)
            .configure(ProfessoresControllersGroup::register)
            .configure(AcoesControllersGroup::register)
            .configure(ControllerVagas::register);
    }
}
