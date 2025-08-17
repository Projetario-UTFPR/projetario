use actix_web::HttpResponse;
use actix_web::web::Redirect;
use comum::erros::erro_de_dominio::ErroDeDominio;

pub mod autenticacao;
pub mod professores;
pub mod projetos;

pub type RespostaDoApp<T = HttpResponse> = Result<T, ErroDeDominio>;
pub type RedirectDoApp = Redirect;
