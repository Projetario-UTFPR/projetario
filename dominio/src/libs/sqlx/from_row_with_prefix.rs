use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::identidade::entidades::professor::builder::ProfessorBuilder;
use crate::identidade::entidades::usuario::builder::UsuarioBuilder;
use crate::projetos::entidades::projeto::builder::ProjetoBuilder;

impl ProfessorBuilder {
    pub fn from_row_with_prefix(row: &PgRow, prefix: &str) -> sqlx::Result<Self> {
        Ok(Self {
            cargo: row.try_get(format!("{prefix}cargo").as_str())?,
            usuario: UsuarioBuilder {
                id: row.try_get(format!("{prefix}id").as_str())?,
                nome: row.try_get(format!("{prefix}nome").as_str())?,
                email: row.try_get(format!("{prefix}email").as_str())?,
                senha_hash: row.try_get(format!("{prefix}senha_hash").as_str())?,
                url_curriculo_lattes: row
                    .try_get(format!("{prefix}url_curriculo_lattes").as_str())?,
                atualizado_em: row.try_get(format!("{prefix}atualizado_em").as_str())?,
                desativado_em: row.try_get(format!("{prefix}desativado_em").as_str())?,
                registrado_em: row.try_get(format!("{prefix}registrado_em").as_str())?,
            },
        })
    }
}

impl ProjetoBuilder {
    pub fn from_row_with_prefix(row: &PgRow, prefix: &str) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get(format!("{prefix}id").as_str())?,
            titulo: row.try_get(format!("{prefix}titulo").as_str())?,
            descricao: row.try_get(format!("{prefix}descricao").as_str())?,
            tipo: row.try_get(format!("{prefix}tipo").as_str())?,
            registrado_em: row.try_get(format!("{prefix}registrado_em").as_str())?,
            iniciado_em: row.try_get(format!("{prefix}iniciado_em").as_str())?,
            concluido_em: row.try_get(format!("{prefix}concluido_em").as_str())?,
            cancelado_em: row.try_get(format!("{prefix}cancelado_em").as_str())?,
            atualizado_em: row.try_get(format!("{prefix}atualizado_em").as_str())?,
        })
    }
}
