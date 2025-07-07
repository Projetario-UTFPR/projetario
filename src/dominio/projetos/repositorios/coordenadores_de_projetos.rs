use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::vec;

use async_trait::async_trait;
use futures_util::FutureExt;
use sqlx::{AnyPool, Connection, Executor, FromRow, PgPool, Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub enum Ordenador {
    Data(Ordering),
    Titulo(Ordering),
}
pub enum Filtro {
    Titulo(String),
    TipoProjeto(TipoDeProjeto),
}
pub struct Paginacao {
    pub pagina: u32,
    pub qtd_por_pagina: u8,
}
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

    async fn buscar_projetos(
        &self,
        filtro: Filtro,
        ordenador: Ordenador,
        paginacao: Paginacao,
    ) -> Result<ProjetosPaginados, ErroDeDominio>;
}
