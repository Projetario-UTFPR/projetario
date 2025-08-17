use serde::Deserialize;
use validator::Validate;

use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::filtragem::{FiltroDeProjeto, OrdenacaoDeProjeto};

#[derive(Deserialize, Validate)]
pub struct BuscarProjetoDto {
    pub filtro: Option<FiltroDeProjeto>,
    pub tipo: Option<TipoDeProjeto>,
    pub ordenador: Option<OrdenacaoDeProjeto>,
    pub pagina: Option<u32>,
    pub qtd_por_pagina: Option<u8>,
}
