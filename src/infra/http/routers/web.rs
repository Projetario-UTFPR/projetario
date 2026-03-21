use inertia_rust::InertiaService;

use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::acoes::AcoesControllersGroup;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::ProfessoresControllersGroup;

pub struct WebRouter;

impl RouterRegistrable for WebRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.inertia_route("/", "index")
            .inertia_route("/dev/hello/world", "hello-world") // rota usada nos testes
            .configure(ControllerAutenticacao::register)
            .configure(ProfessoresControllersGroup::register)
            .configure(AcoesControllersGroup::register);
    }
}
