use actix_web::ResponseError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::TryIntoHeaderValue;
use comum::erros::{ErroDeDominio, TipoErroDeDominio};
use serde::Serialize;

pub mod server;
