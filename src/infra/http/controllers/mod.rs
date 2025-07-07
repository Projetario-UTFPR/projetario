use actix_web::HttpResponse;
use actix_web::web::{Redirect, ServiceConfig};

use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub mod autenticacao;
pub mod professores;

pub trait Controller {
    fn register(cfg: &mut ServiceConfig);
}

pub type RespostaDoApp<T = HttpResponse> = Result<T, ErroDeDominio>;
pub type RedirectDoApp = Redirect;
