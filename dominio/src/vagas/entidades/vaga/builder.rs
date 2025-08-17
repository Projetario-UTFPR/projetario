use chrono::NaiveDate;
use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::projetos::agregados::projeto_com_coordenadores::builder::ProjetoComCoordenadoresBuilder;
use crate::vagas::entidades::vaga::Vaga;

pub struct VagaBuilder {
    pub id: Uuid,
    pub projeto_e_coordenadores: ProjetoComCoordenadoresBuilder,
    pub horas_por_semana: u8,
    pub imagem: String,
    pub quantidade: u8,
    pub link_edital: String,
    pub link_candidatura: Option<String>,
    pub titulo: Option<String>,
    pub conteudo: String,
    pub iniciada_em: NaiveDate,
    pub inscricoes_ate: DbDateTime,
    pub cancelada_em: Option<DbDateTime>,
    pub atualizada_em: Option<DbDateTime>,
}

impl From<VagaBuilder> for Vaga {
    fn from(value: VagaBuilder) -> Self {
        Self {
            atualizada_em: value.atualizada_em,
            cancelada_em: value.cancelada_em,
            conteudo: value.conteudo,
            projeto_e_coordenadores: value.projeto_e_coordenadores.into(),
            horas_por_semana: value.horas_por_semana,
            id: value.id,
            imagem: value.imagem,
            iniciada_em: value.iniciada_em,
            inscricoes_ate: value.inscricoes_ate,
            link_candidatura: value.link_candidatura,
            link_edital: value.link_edital,
            quantidade: value.quantidade,
            titulo: value.titulo,
        }
    }
}
