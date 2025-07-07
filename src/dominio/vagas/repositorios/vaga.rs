use async_trait::async_trait;

use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[async_trait]
pub trait RepositorioDeVagas {
    async fn criar_vaga(&self, vaga: &Vaga) -> Result<(), ErroDeDominio>;
}
