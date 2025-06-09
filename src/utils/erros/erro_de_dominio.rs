use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErroDeDominio {
    Integridade(String),
    ValorInvalido(String),
}

impl Display for ErroDeDominio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match &self {
            Self::ValorInvalido(msg) => msg,
            Self::Integridade(msg) => msg,
        };

        write!(f, "{msg}",)
    }
}
