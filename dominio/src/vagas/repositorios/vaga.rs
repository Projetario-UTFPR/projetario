use std::collections::HashSet;

use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use uuid::Uuid;

use crate::comum::paginacao::{EntidadePaginada, Paginacao};
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::filtragem::{FiltroDeVaga, OrdenacaoDeVaga};

#[async_trait]
pub trait RepositorioDeVagas {
    async fn criar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
    async fn buscar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Vaga>>;
    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
    async fn buscar_vagas(
        &self,
        filtros: HashSet<FiltroDeVaga>,
        ordenador: OrdenacaoDeVaga,
        paginacao: Paginacao,
    ) -> ResultadoDominio<EntidadePaginada<Vaga>>;
}
