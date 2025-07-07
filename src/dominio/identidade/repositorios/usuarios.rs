use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::utils::erros::ResultadoDominio;

#[async_trait]
pub trait RepositorioDeUsuarios {
    async fn encontre_professor_pelo_email(
        &self,
        email: &str,
    ) -> ResultadoDominio<Option<Professor>>;

    async fn encontre_aluno_pelo_ra(&self, ra: &str) -> ResultadoDominio<Option<Aluno>>;

    async fn encontre_usuario_modelo_pelo_id(
        &self,
        id: &Uuid,
    ) -> ResultadoDominio<Option<UsuarioModelo>>;
}
