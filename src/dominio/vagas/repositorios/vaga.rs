use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[async_trait]
pub trait RepositorioDeVagas {
    async fn criar_vaga(&self, vaga: &Vaga) -> Result<(), ErroDeDominio>;
    async fn buscar_por_id(&self, id: Uuid) -> Result<Option<Vaga>, ErroDeDominio>;
    async fn atualizar_vaga(&self, vaga: &Vaga) -> Result<(), ErroDeDominio>;
    async fn cancelar_vaga(&self, id: Uuid) -> Result<(), ErroDeDominio>;
}
