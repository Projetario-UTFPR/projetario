use std::fmt::Display;

use thiserror::Error;

use crate::utils::erros::tipo_erro_de_dominio::TipoErroDeDominio;

#[derive(Debug, Error)]
pub struct ErroDeDominio {
    tipo_: TipoErroDeDominio,
    msg: String,
}

impl ErroDeDominio {
    pub fn novo(tipo: TipoErroDeDominio, msg: String) -> Self { Self { tipo_: tipo, msg } }

    pub fn integridade<S: ToString>(msg: S) -> Self {
        Self {
            tipo_: TipoErroDeDominio::Integridade,
            msg: msg.to_string(),
        }
    }

    pub fn valor_invalido<S: ToString>(msg: S) -> Self {
        Self {
            tipo_: TipoErroDeDominio::ValorInvalido,
            msg: msg.to_string(),
        }
    }

    pub fn interno() -> Self {
        Self {
            tipo_: TipoErroDeDominio::Interno,
            msg: "Houve um problema no nosso servidor.".to_string(),
        }
    }

    pub fn com_mensagem<T: ToString>(mut self, msg: T) -> Self {
        self.msg = msg.to_string();
        self
    }

    pub fn mensagem(&self) -> &str { &self.msg }

    pub fn tipo(&self) -> TipoErroDeDominio { self.tipo_.clone() }

    pub fn nao_encontrado<S: ToString>(msg: S) -> Self {
        Self {
            tipo_: TipoErroDeDominio::NaoEncontrado,
            msg: msg.to_string(),
        }
    }

    pub fn nao_autorizado<S: ToString>(msg: S) -> Self {
        Self {
            tipo_: TipoErroDeDominio::NaoAutorizado,
            msg: msg.to_string(),
        }
    }

    pub fn regra_de_negocio<S: ToString>(msg: S) -> Self {
        Self {
            tipo_: TipoErroDeDominio::RegraDeNegocio,
            msg: msg.to_string(),
        }
    }
}

impl Display for ErroDeDominio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.msg) }
}
