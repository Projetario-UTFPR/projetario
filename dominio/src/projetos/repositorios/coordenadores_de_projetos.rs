use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use comum::erros::erro_de_dominio::ErroDeDominio;
use uuid::Uuid;

use crate::identidade::entidades::professor::Professor;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::entidades::projeto::Projeto;

#[async_trait]
pub trait RepositorioDeCoordenadoresDeProjetos {
    /// Persiste o `projeto` e, imediatamente, associa-o com o professor responsável por ele.
    async fn criar_projeto_com_coordenador(
        &self,
        projeto: &Projeto,
        coordenador: &Professor,
    ) -> Result<(), ErroDeDominio>;

    async fn buscar_projeto_e_coordenadores_por_id(
        &self,
        id_projeto: &Uuid,
    ) -> ResultadoDominio<Option<ProjetoComCoordenadores>>;
}
