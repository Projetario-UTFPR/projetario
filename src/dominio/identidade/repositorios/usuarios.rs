use async_trait::async_trait;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::utils::erros::ResultadoDominio;

#[async_trait]
pub trait RepositorioDeUsuarios {
    async fn encontre_professor_pelo_email(
        &self,
        email: &str,
    ) -> ResultadoDominio<Option<Professor>>;

    async fn encontre_aluno_pelo_ra(&self, ra: &str) -> ResultadoDominio<Option<Aluno>>;
}
