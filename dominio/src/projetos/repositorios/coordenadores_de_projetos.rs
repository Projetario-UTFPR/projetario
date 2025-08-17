use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use comum::erros::erro_de_dominio::ErroDeDominio;
use serde::Serialize;
use uuid::Uuid;

use crate::comum::paginacao::Paginacao;
use crate::identidade::entidades::professor::Professor;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::projetos::filtragem::{FiltroDeProjeto, OrdenacaoDeProjeto};

#[derive(Serialize)]
pub struct ProjetosPaginados {
    pub projetos: Vec<Projeto>,
    pub qtd_por_pagina: u8,
}
#[async_trait]
pub trait RepositorioDeCoordenadoresDeProjetos {
    /// Persiste o `projeto` e, imediatamente, associa-o com o professor responsável por ele.
    async fn criar_projeto_com_coordenador(
        &self,
        projeto: &Projeto,
        coordenador: &Professor,
    ) -> Result<(), ErroDeDominio>;

    async fn buscar_projeto_e_coordenadores_por_id(
        &self,
        id_projeto: &Uuid,
    ) -> ResultadoDominio<Option<ProjetoComCoordenadores>>;

    async fn buscar_projetos(
        &self,
        filtro: Option<FiltroDeProjeto>,
        tipo: Option<TipoDeProjeto>,
        ordenador: OrdenacaoDeProjeto,
        paginacao: Paginacao,
    ) -> Result<ProjetosPaginados, ErroDeDominio>;
}
