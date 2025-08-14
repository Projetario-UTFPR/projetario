use actix_web::web;

use crate::dominio::identidade::enums::cargo::Cargo;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::autenticacao::ControllerAutenticacao;
use crate::infra::http::controllers::professores::projetos_de_extensao::ControllerProjetosDeExtensao;
use crate::infra::http::controllers::professores::vagas::ControllerVagas;
use crate::infra::http::middlewares::somente_com_cargo::{
    AutorizacaoDaRota,
    MiddlewareEstaAutorizado,
};

pub struct WebRouter;

impl RouterRegistrable for WebRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.configure(ControllerAutenticacao::register);
    }
}
