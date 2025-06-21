use actix_web::ResponseError;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::http::header::TryIntoHeaderValue;
use serde::Serialize;

use crate::utils::erros::{ErroDeDominio, TipoErroDeDominio};

pub mod server;

#[derive(Serialize)]
struct ErroDeDominioEmJson {
    mensagem: String,
    codigo: String,
}

impl ResponseError for ErroDeDominio {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.tipo() {
            TipoErroDeDominio::Integridade => StatusCode::FORBIDDEN,
            TipoErroDeDominio::Interno => StatusCode::INTERNAL_SERVER_ERROR,
            TipoErroDeDominio::NãoAutorizado => StatusCode::UNAUTHORIZED,
            TipoErroDeDominio::ValorInvalido => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut res = actix_web::HttpResponse::new(self.status_code());

        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            actix_web::mime::APPLICATION_JSON.try_into_value().unwrap(),
        );

        let erro_json = serde_json::to_string(&ErroDeDominioEmJson {
            codigo: self.tipo().como_codigo(),
            mensagem: self.mensagem().into(),
        })
        .unwrap();

        res.set_body(BoxBody::new(erro_json))
    }
}
