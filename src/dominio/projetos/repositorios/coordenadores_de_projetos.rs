use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::vec;

use async_trait::async_trait;
use futures_util::FutureExt;
use serde::{Deserialize, Serialize};
use sqlx::{AnyPool, Connection, Executor, FromRow, PgPool, Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::comum::filtragem::DirecaoOrdenacao;
use crate::comum::paginacao::Paginacao;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::filtragem::{FiltroDeProjeto, OrdenacaoDeProjeto};
use crate::utils::erros::ResultadoDominio;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

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
