use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::utils::erros::ResultadoDominio;

#[async_trait]
pub trait RepositorioDeProjetos {
    async fn encontrar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Projeto>>;
}
