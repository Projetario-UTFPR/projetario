use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use uuid::Uuid;

use crate::identidade::entidades::aluno::Aluno;
use crate::identidade::entidades::professor::Professor;
use crate::identidade::entidades::usuario::UsuarioModelo;

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
