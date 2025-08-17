use async_trait::async_trait;
use comum::erros::{ErroDeDominio, ResultadoDominio};
use uuid::Uuid;

use crate::comum::paginacao::{EntidadePaginada, Paginacao};
use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::projetos::filtragem::{EstadoDoProjeto, FiltroDeProjeto, OrdenacaoDeProjeto};

#[async_trait]
pub trait RepositorioDeProjetos {
    async fn encontrar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Projeto>>;

    async fn buscar_projetos(
        &self,
        filtro: Option<FiltroDeProjeto>,
        estado: Option<EstadoDoProjeto>,
        tipo: Option<TipoDeProjeto>,
        ordenador: OrdenacaoDeProjeto,
        paginacao: Paginacao,
    ) -> Result<EntidadePaginada<Projeto>, ErroDeDominio>;
}
