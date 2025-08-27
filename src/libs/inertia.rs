use comum::erros::{ErroDeDominio, ResultadoDominio};
use inertia_rust::{InertiaError, IntoInertiaPropResult};
use serde::Serialize;
use serde_json::Value;

use crate::infra::http::resposta_incerta_do_servidor::RespostaIncertaDoServidor;

pub mod middleware;

pub fn inertiafy_domain_error<T: Serialize>(
    data: ResultadoDominio<T>,
) -> Result<Value, InertiaError> {
    flatten_inertia_error(data.map(|data| data.into_inertia_value()))
}

pub fn flatten_inertia_error<T: Serialize>(
    data: Result<Result<T, InertiaError>, ErroDeDominio>,
) -> Result<Value, InertiaError> {
    match data {
        Err(err) => RespostaIncertaDoServidor {
            success: false,
            error: Some(err.to_string()),
            data: None,
        },
        Ok(data) => match data {
            Err(err) => {
                log::error!("{err}");
                RespostaIncertaDoServidor {
                    success: false,
                    error: Some(ErroDeDominio::interno().to_string()),
                    data: None,
                }
            }
            Ok(data) => RespostaIncertaDoServidor {
                success: true,
                error: None,
                data: Some(data),
            },
        },
    }
    .into_inertia_value()
}
