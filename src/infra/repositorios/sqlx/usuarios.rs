use async_trait::async_trait;
use sqlx::{Execute, PgPool, query_as};
use uuid::Uuid;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};

pub struct RepositorioDeUsuariosSQLX<'this> {
    db_conn: &'this PgPool,
}

impl<'this> RepositorioDeUsuariosSQLX<'this> {
    pub fn novo(db_conn: &'this PgPool) -> Self { Self { db_conn } }
}

#[async_trait]
impl RepositorioDeUsuarios for RepositorioDeUsuariosSQLX<'_> {
    async fn encontre_professor_pelo_email(
        &self,
        email: &str,
    ) -> ResultadoDominio<Option<Professor>> {
        let query = query_as(
            "SELECT \
            id, nome, email, senha_hash, url_curriculo_lattes, registrado_em, \
            atualizado_em, desativado_em, cargo \
            FROM \"usuario\" \
            WHERE email = $1
            AND cargo in ('professor', 'administrador') \
            AND registro_aluno IS NULL \
            AND periodo IS NULL
            LIMIT 1",
        )
        .bind(email);

        query.fetch_optional(self.db_conn).await.map_err(|err| {
            log::error!("Erro ao buscar um professor por email: {err}");
            ErroDeDominio::interno()
        })
    }

    async fn encontre_aluno_pelo_ra(&self, ra: &str) -> ResultadoDominio<Option<Aluno>> {
        let query = query_as(
            "SELECT \
            id, nome, email, senha_hash, url_curriculo_lattes, registrado_em, \
            atualizado_em, desativado_em, registro_aluno, periodo \
            FROM \"usuario\" \
            WHERE registro_aluno = $1
            AND cargo = 'aluno' \
            AND periodo IS NOT NULL
            LIMIT 1",
        )
        .bind(ra);

        query.fetch_optional(self.db_conn).await.map_err(|err| {
            log::error!("Erro ao buscar um aluno por ra: {err}");
            ErroDeDominio::interno()
        })
    }

    async fn encontre_usuario_modelo_pelo_id(
        &self,
        id: &Uuid,
    ) -> ResultadoDominio<Option<UsuarioModelo>> {
        let query = query_as(
            "SELECT \
            id, nome, email, senha_hash, url_curriculo_lattes, registrado_em, \
            atualizado_em, desativado_em, registro_aluno, periodo, cargo \
            FROM \"usuario\" \
            WHERE id = $1
            LIMIT 1
            ",
        )
        .bind(id);

        query.fetch_optional(self.db_conn).await.map_err(|err| {
            log::error!("Erro ao buscar um usuário pelo seu id: {err}");
            ErroDeDominio::interno()
        })
    }
}
