use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use serde::Deserialize;
use validator::Validate;

use crate::infra::dtos::projetos::filtro::FiltroDto;
use crate::infra::dtos::projetos::ordenacao::OrdenacaoDto;

#[derive(Deserialize, Validate)]
pub struct BuscarProjetoDto {
    pub filtro: FiltroDto,
    pub ordenacao: OrdenacaoDto,
    pub tipo: Option<TipoDeProjeto>,
    pub pagina: Option<u32>,
    pub qtd_por_pagina: Option<u8>,
}
