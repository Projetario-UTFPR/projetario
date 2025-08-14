use actix_web::web;

use crate::dominio::identidade::enums::cargo::Cargo;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::professores::projetos_de_extensao::ControllerProjetosDeExtensao;
use crate::infra::http::controllers::professores::vagas::ControllerVagas;
use crate::infra::http::middlewares::somente_com_cargo::{
    AutorizacaoDaRota,
    MiddlewareEstaAutorizado,
};

pub mod projetos_de_extensao;
pub mod vagas;

pub struct ProfessoresRouter;

impl RouterRegistrable for ProfessoresRouter {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/professores")
                .wrap(MiddlewareEstaAutorizado::novo(
                    AutorizacaoDaRota::UsuarioComCargo(Cargo::Professor),
                ))
                .configure(ControllerProjetosDeExtensao::register)
                .configure(ControllerVagas::register),
        );
    }
}
