use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;

use crate::identidade::entidades::professor::builder::ProfessorBuilder;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::agregados::projeto_com_coordenadores::builder::ProjetoComCoordenadoresBuilder;
use crate::projetos::entidades::projeto::builder::ProjetoBuilder;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::entidades::vaga::builder::VagaBuilder;

impl<'r> FromRow<'r, PgRow> for Vaga {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let vice_id: Option<Uuid> = row.try_get("vice_id")?;
        let builder = VagaBuilder {
            id: row.try_get("vaga_id")?,
            imagem: row.try_get("vaga_imagem")?,
            conteudo: row.try_get("vaga_contudo")?,
            horas_por_semana: row.try_get::<'r, i32, &'r str>("vaga_horas_por_semana")? as u8,
            link_edital: row.try_get("vaga_link_edital")?,
            link_candidatura: row.try_get("vaga_link_candidatura")?,
            quantidade: row.try_get::<'r, i32, &'r str>("vaga_quantidade")? as u8,
            titulo: row.try_get("vaga_titulo")?,
            inscricoes_ate: row.try_get("vaga_inscricoes_ate")?,
            iniciada_em: row.try_get("vaga_iniciada_em")?,
            atualizada_em: row.try_get("vaga_atualizada_em")?,
            cancelada_em: row.try_get("vaga_cancelada_em")?,
            projeto_e_coordenadores: ProjetoComCoordenadoresBuilder {
                projeto: ProjetoBuilder::from_row_with_prefix(row, "p")?,
                coordenador: ProfessorBuilder::from_row_with_prefix(row, "coord_")?,
                vice_coordenador: match vice_id {
                    None => None,
                    Some(_) => Some(ProfessorBuilder::from_row_with_prefix(row, "vice_")?),
                },
            },
        };

        Ok(builder.into())
    }
}

impl<'r> FromRow<'r, PgRow> for ProjetoComCoordenadores {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let vice_coordenador_id: Option<Uuid> = row.try_get("vice_id")?;
        let builder = ProjetoComCoordenadoresBuilder {
            projeto: ProjetoBuilder {
                id: row.try_get("id")?,
                titulo: row.try_get("titulo")?,
                descricao: row.try_get("descricao")?,
                tipo: row.try_get("tipo")?,
                registrado_em: row.try_get("registrado_em")?,
                iniciado_em: row.try_get("iniciado_em")?,
                atualizado_em: row.try_get("atualizado_em")?,
                cancelado_em: row.try_get("cancelado_em")?,
                concluido_em: row.try_get("concluido_em")?,
            },
            coordenador: ProfessorBuilder::from_row_with_prefix(row, "coorde_")?,
            vice_coordenador: match vice_coordenador_id {
                None => None,
                Some(_) => Some(ProfessorBuilder::from_row_with_prefix(row, "vice_")?),
            },
        };

        Ok(builder.into())
    }
}
