use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::identidade::entidades::usuario::Usuario;

pub struct UsuarioBuilder {
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha_hash: String,
    pub url_curriculo_lattes: Option<String>,
    pub registrado_em: DbDateTime,
    pub atualizado_em: Option<DbDateTime>,
    pub desativado_em: Option<DbDateTime>,
}

impl From<UsuarioBuilder> for Usuario {
    fn from(value: UsuarioBuilder) -> Self {
        Self {
            atualizado_em: value.atualizado_em,
            desativado_em: value.desativado_em,
            email: value.email,
            id: value.id,
            nome: value.nome,
            registrado_em: value.registrado_em,
            senha_hash: value.senha_hash,
            url_curriculo_lattes: value.url_curriculo_lattes,
        }
    }
}
