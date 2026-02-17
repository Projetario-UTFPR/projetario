use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use uuid::Uuid;

use crate::identidade::entidades::professor::Professor;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::entidades::projeto::Projeto;
use crate::vagas::entidades::vaga::Vaga;

impl<'r> FromRow<'r, PgRow> for Vaga {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let vice_id: Option<Uuid> = row.try_get("vice_id")?;

        let projeto_e_coordenadores = ProjetoComCoordenadores::novo(
            Projeto::from_row_with_prefix(row, "p_")?,
            Professor::from_row_with_prefix(row, "coord_")?,
            match vice_id {
                None => None,
                Some(_) => Some(Professor::from_row_with_prefix(row, "vice_")?),
            },
        );

        let vaga = Vaga::nova_de_dados_brutos(
            row.try_get("vaga_id")?,
            projeto_e_coordenadores,
            row.try_get::<'r, i16, &'r str>("vaga_horas_por_semana")? as u8,
            row.try_get("vaga_imagem")?,
            row.try_get::<'r, i16, &'r str>("vaga_quantidade")? as u8,
            row.try_get("vaga_link_edital")?,
            row.try_get("vaga_link_candidatura")?,
            row.try_get("vaga_titulo")?,
            row.try_get("vaga_conteudo")?,
            row.try_get("vaga_iniciada_em")?,
            row.try_get("vaga_inscricoes_ate")?,
            row.try_get("vaga_cancelada_em")?,
            row.try_get("vaga_atualizada_em")?,
        );

        Ok(vaga)
    }
}

impl<'r> FromRow<'r, PgRow> for ProjetoComCoordenadores {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let coordenador = Professor::from_row_with_prefix(row, "coorde_")?;

        let vice_coordenador_id: Option<Uuid> = row.try_get("vice_id")?;
        let vice_coordenador = match vice_coordenador_id {
            None => None,
            Some(_) => Some(Professor::from_row_with_prefix(row, "vice_")?),
        };

        let projeto = Projeto::novo_de_dados_brutos(
            row.try_get("id")?,
            row.try_get("titulo")?,
            row.try_get("descricao")?,
            row.try_get("tipo")?,
            row.try_get("registrado_em")?,
            row.try_get("iniciado_em")?,
            row.try_get("atualizado_em")?,
            row.try_get("cancelado_em")?,
            row.try_get("concluido_em")?,
        );

        Ok(ProjetoComCoordenadores::novo(
            projeto,
            coordenador,
            vice_coordenador,
        ))
    }
}
