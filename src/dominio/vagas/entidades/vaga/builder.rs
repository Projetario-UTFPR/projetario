use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::builder::ProfessorBuilder;
use crate::dominio::projetos::entidades::projeto::builder::ProjetoBuilder;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::utils::sqlx::DbDateTime;

pub struct VagaBuilder {
    pub id: Uuid,
    pub projeto: ProjetoBuilder,
    pub coordenador: ProfessorBuilder,
    pub vice_coordenador: Option<ProfessorBuilder>,
    pub horas_por_semana: u8,
    pub imagem: String,
    pub quantidade: u8,
    pub link_edital: String,
    pub link_candidatura: Option<String>,
    pub titulo: Option<String>,
    pub conteudo: String,
    pub iniciada_em: DbDateTime,
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
            coordenador: value.coordenador.into(),
            horas_por_semana: value.horas_por_semana,
            id: value.id,
            imagem: value.imagem,
            iniciada_em: value.iniciada_em,
            inscricoes_ate: value.inscricoes_ate,
            link_candidatura: value.link_candidatura,
            link_edital: value.link_edital,
            projeto: value.projeto.into(),
            quantidade: value.quantidade,
            titulo: value.titulo,
            vice_coordenador: value.vice_coordenador.map(|vice| vice.into()),
        }
    }
}
