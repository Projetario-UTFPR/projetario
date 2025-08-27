use serde::Deserialize;

use crate::infra::tema::TemaDaAplicacaoEstrito;

#[derive(Deserialize)]
pub struct TemaEstritoDto {
    pub tema: TemaDaAplicacaoEstrito,
}
