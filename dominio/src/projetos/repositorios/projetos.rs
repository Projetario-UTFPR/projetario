use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use uuid::Uuid;

use crate::projetos::entidades::projeto::Projeto;

#[async_trait]
pub trait RepositorioDeProjetos {
    async fn encontrar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Projeto>>;
}
