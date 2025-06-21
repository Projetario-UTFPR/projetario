use async_trait::async_trait;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[async_trait]
pub trait RepositorioDeUsuarios {
    async fn encontre_professor_pelo_email(
        &self,
        email: &str,
    ) -> Result<Option<Professor>, ErroDeDominio>;

    async fn encontre_aluno_pelo_ra(&self, ra: &str) -> Result<Option<Aluno>, ErroDeDominio>;
}
