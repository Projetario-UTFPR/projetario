use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::utils::erros::ResultadoDominio;

#[async_trait]
pub trait RepositorioDeVagas {
    async fn criar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
    async fn buscar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Vaga>>;
    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
}
