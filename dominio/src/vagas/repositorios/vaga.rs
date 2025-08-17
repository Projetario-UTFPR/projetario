use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use uuid::Uuid;

use crate::vagas::entidades::vaga::Vaga;

#[async_trait]
pub trait RepositorioDeVagas {
    async fn criar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
    async fn buscar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Vaga>>;
    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()>;
}
