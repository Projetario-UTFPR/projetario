use std::fmt::Display;

use comum::erros::{ErroDeDominio, ResultadoDominio};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemaDaAplicacao {
    #[default]
    Sistema,
    Escuro,
    Claro,
}

#[derive(Default, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TemaDaAplicacaoEstrito {
    #[default]
    Escuro,
    Claro,
}

impl TemaDaAplicacaoEstrito {
    pub fn seguinte(self) -> Self {
        match self {
            Self::Claro => Self::Escuro,
            _ => Self::Claro,
        }
    }
}

impl TemaDaAplicacao {
    pub fn seguinte(self) -> Self {
        match self {
            Self::Claro => Self::Escuro,
            Self::Escuro => Self::Sistema,
            Self::Sistema => Self::Claro,
        }
    }
}

impl From<TemaDaAplicacaoEstrito> for TemaDaAplicacao {
    fn from(value: TemaDaAplicacaoEstrito) -> Self {
        match value {
            TemaDaAplicacaoEstrito::Claro => TemaDaAplicacao::Claro,
            TemaDaAplicacaoEstrito::Escuro => TemaDaAplicacao::Escuro,
        }
    }
}

impl TryFrom<&str> for TemaDaAplicacao {
    type Error = ErroDeDominio;

    fn try_from(value: &str) -> ResultadoDominio<Self> {
        match value {
            "claro" => Ok(TemaDaAplicacao::Claro),
            "escuro" => Ok(TemaDaAplicacao::Escuro),
            "sistema" => Ok(TemaDaAplicacao::Sistema),
            _ => Err(ErroDeDominio::valor_invalido(
                "O tema só pode assumir um dos valores: 'claro', 'escuro' e 'sistema'.",
            )),
        }
    }
}

impl TryFrom<&str> for TemaDaAplicacaoEstrito {
    type Error = ErroDeDominio;

    fn try_from(value: &str) -> ResultadoDominio<Self> {
        match value {
            "claro" => Ok(TemaDaAplicacaoEstrito::Claro),
            "escuro" => Ok(TemaDaAplicacaoEstrito::Escuro),
            _ => Err(ErroDeDominio::valor_invalido(
                "O tema só pode assumir um dos valores: 'claro', 'escuro' e 'sistema'.",
            )),
        }
    }
}

impl Display for TemaDaAplicacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TemaDaAplicacao::Claro => "claro",
                TemaDaAplicacao::Escuro => "escuro",
                TemaDaAplicacao::Sistema => "sistema",
            }
        )
    }
}

impl Display for TemaDaAplicacaoEstrito {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", TemaDaAplicacao::from(self.clone()))
    }
}
