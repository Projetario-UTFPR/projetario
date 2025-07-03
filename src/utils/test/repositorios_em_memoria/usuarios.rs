use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::utils::erros::ResultadoDominio;
use crate::utils::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;

pub struct RepositorioDeUsuariosEmMemoria {
    pub usuarios_tbl: TabelaThreadSafeEmMemoria<UsuarioModelo>,
}

#[async_trait]
impl RepositorioDeUsuarios for RepositorioDeUsuariosEmMemoria {
    async fn encontre_professor_pelo_email(
        &self,
        email: &str,
    ) -> ResultadoDominio<Option<Professor>> {
        Ok(self
            .usuarios_tbl
            .lock()
            .unwrap()
            .iter()
            .find(|usuario| {
                usuario.cargo != Cargo::Aluno
                    && usuario.registro_aluno.is_none()
                    && usuario.periodo.is_none()
            })
            .map(TryFrom::try_from)
            .map(Result::unwrap))
    }

    async fn encontre_aluno_pelo_ra(&self, ra: &str) -> ResultadoDominio<Option<Aluno>> {
        Ok(self
            .usuarios_tbl
            .lock()
            .unwrap()
            .iter()
            .find(|usuario| {
                usuario.cargo == Cargo::Aluno
                    && usuario.periodo.is_some()
                    && usuario.registro_aluno.is_some()
            })
            .map(TryFrom::try_from)
            .map(Result::unwrap))
    }

    async fn encontre_usuario_modelo_pelo_id(
        &self,
        id: &Uuid,
    ) -> ResultadoDominio<Option<UsuarioModelo>> {
        Ok(self
            .usuarios_tbl
            .lock()
            .unwrap()
            .iter()
            .find(|usuario| usuario.id.eq(id))
            .cloned())
    }
}
