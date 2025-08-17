use chrono::NaiveDate;
use comum::sqlx::{DbDateTime, db_date_time_now};
use fake::Fake;
use fake::faker::job::pt_br::Title;
use fake::faker::lorem;
use uuid::Uuid;

use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::entidades::projeto::builder::ProjetoBuilder;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;

#[allow(unused)]
#[derive(Default)]
pub struct ProjetoParcial {
    id: Option<Uuid>,
    titulo: Option<String>,
    descricao: Option<String>,
    tipo: Option<TipoDeProjeto>,
    registrado_em: Option<DbDateTime>,
    iniciado_em: Option<NaiveDate>,
    atualizado_em: Option<DbDateTime>,
    cancelado_em: Option<DbDateTime>,
    concluido_em: Option<NaiveDate>,
}

impl ProjetoParcial {
    pub fn into_builder(self) -> ProjetoBuilder {
        ProjetoBuilder {
            id: self.id.unwrap_or_else(Uuid::new_v4),
            titulo: self.titulo.unwrap_or_else(|| Title().fake()),
            descricao: self.descricao.unwrap_or_else(|| {
                lorem::pt_br::Paragraphs(2..5)
                    .fake::<Vec<String>>()
                    .join("\n")
            }),
            tipo: TipoDeProjeto::Extensao,
            registrado_em: db_date_time_now(),
            iniciado_em: db_date_time_now().date(),
            atualizado_em: None,
            cancelado_em: None,
            concluido_em: None,
        }
    }

    pub fn into_entidade(self) -> Projeto { self.into_builder().into() }
}
