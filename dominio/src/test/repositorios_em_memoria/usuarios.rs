use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use uuid::Uuid;

use crate::identidade::entidades::aluno::Aluno;
use crate::identidade::entidades::professor::Professor;
use crate::identidade::entidades::usuario::UsuarioModelo;
use crate::identidade::enums::cargo::Cargo;
use crate::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;

#[derive(Clone)]
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
                    && usuario.email == email
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
                    && usuario
                        .registro_aluno
                        .as_ref()
                        .is_some_and(|registro_aluno| registro_aluno == ra)
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
