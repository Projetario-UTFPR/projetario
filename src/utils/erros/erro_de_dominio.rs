use std::fmt::Display;

use thiserror::Error;

#[derive(Debug)]
pub enum TipoErroDeDominio {
    Integridade,
    ValorInvalido,
    NãoAutorizado,
}

#[derive(Debug, Error)]
pub struct ErroDeDominio {
    tipo: TipoErroDeDominio,
    msg: String,
}

impl ErroDeDominio {
    pub fn novo(tipo: TipoErroDeDominio, msg: String) -> Self {
        Self { tipo, msg }
    }

    pub fn integridade<S: ToString>(msg: S) -> Self {
        Self {
            tipo: TipoErroDeDominio::Integridade,
            msg: msg.to_string(),
        }
    }

    pub fn valor_invalido<S: ToString>(msg: S) -> Self {
        Self {
            tipo: TipoErroDeDominio::ValorInvalido,
            msg: msg.to_string(),
        }
    }

    pub fn nao_autorizado<S: ToString>(msg: S) -> Self {
        Self {
            tipo: TipoErroDeDominio::NãoAutorizado,
            msg: msg.to_string(),
        }
    }
}

impl Display for ErroDeDominio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
