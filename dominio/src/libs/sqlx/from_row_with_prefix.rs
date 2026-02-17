use sqlx::Row;
use sqlx::postgres::PgRow;

use crate::identidade::entidades::professor::Professor;
use crate::identidade::entidades::usuario::Usuario;
use crate::projetos::entidades::projeto::Projeto;

impl Professor {
    pub fn from_row_with_prefix(row: &PgRow, prefix: &str) -> sqlx::Result<Self> {
        Ok(Professor::novo_de_dados_brutos(
            Usuario::novo_de_dados_brutos(
                row.try_get(format!("{prefix}id").as_str())?,
                row.try_get(format!("{prefix}nome").as_str())?,
                row.try_get(format!("{prefix}email").as_str())?,
                row.try_get(format!("{prefix}senha_hash").as_str())?,
                row.try_get(format!("{prefix}url_curriculo_lattes").as_str())?,
                row.try_get(format!("{prefix}registrado_em").as_str())?,
                row.try_get(format!("{prefix}atualizado_em").as_str())?,
                row.try_get(format!("{prefix}desativado_em").as_str())?,
            ),
            row.try_get(format!("{prefix}cargo").as_str())?,
        ))
    }
}

impl Projeto {
    pub fn from_row_with_prefix(row: &PgRow, prefix: &str) -> sqlx::Result<Self> {
        Ok(Projeto::novo_de_dados_brutos(
            row.try_get(format!("{prefix}id").as_str())?,
            row.try_get(format!("{prefix}titulo").as_str())?,
            row.try_get(format!("{prefix}descricao").as_str())?,
            row.try_get(format!("{prefix}tipo").as_str())?,
            row.try_get(format!("{prefix}registrado_em").as_str())?,
            row.try_get(format!("{prefix}iniciado_em").as_str())?,
            row.try_get(format!("{prefix}atualizado_em").as_str())?,
            row.try_get(format!("{prefix}cancelado_em").as_str())?,
            row.try_get(format!("{prefix}concluido_em").as_str())?,
        ))
    }
}
