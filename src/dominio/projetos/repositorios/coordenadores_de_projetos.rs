use async_trait::async_trait;

use crate::{
    dominio::{identidade::entidades::professor::Professor, projetos::entidades::projeto::Projeto},
    utils::erros::erro_de_dominio::ErroDeDominio,
};

#[async_trait]
pub trait RepositorioDeCoordenadoresDeProjetos {
    /// Persiste o `projeto` e, imediatamente, associa-o com o professor responsável por ele.
    async fn criar_projeto_com_coordenador(
        &self,
        projeto: &Projeto,
        coordenador: &Professor,
    ) -> Result<(), ErroDeDominio>;
}
