use chrono::NaiveDate;
use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;

pub struct ProjetoBuilder {
    pub id: Uuid,
    pub titulo: String,
    pub descricao: String,
    pub tipo: TipoDeProjeto,
    pub registrado_em: DbDateTime,
    pub iniciado_em: NaiveDate,
    pub atualizado_em: Option<DbDateTime>,
    pub cancelado_em: Option<DbDateTime>,
    pub concluido_em: Option<NaiveDate>,
}

impl From<ProjetoBuilder> for Projeto {
    fn from(value: ProjetoBuilder) -> Self {
        Self {
            atualizado_em: value.atualizado_em,
            cancelado_em: value.cancelado_em,
            concluido_em: value.concluido_em,
            descricao: value.descricao,
            id: value.id,
            iniciado_em: value.iniciado_em,
            registrado_em: value.registrado_em,
            tipo: value.tipo,
            titulo: value.titulo,
        }
    }
}
