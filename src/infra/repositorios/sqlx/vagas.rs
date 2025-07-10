use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};

pub struct RepositorioDeVagasSQLX<'this> {
    db_conn: &'this PgPool,
}

impl<'this> RepositorioDeVagasSQLX<'this> {
    pub fn novo(db_conn: &'this PgPool) -> Self { Self { db_conn } }
}

// TODO: tratar todos os erros internos que podem ser erros de verdade no banco de dados
#[async_trait::async_trait]
impl RepositorioDeVagas for RepositorioDeVagasSQLX<'_> {
    async fn criar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()> {
        sqlx::query(
            "INSERT INTO vaga (\
                id, id_projeto, id_coordenador, id_vice_coordenador, \
                horas_por_semana, imagem, quantidade, link_edital, \
                link_candidatura, titulo, conteudo, iniciada_em, \
                inscricoes_ate, cancelada_em, atualizada_em) \
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, \
                $13, $14, $15
            )",
        )
        .bind(vaga.obtenha_id())
        .bind(vaga.obtenha_projeto().obtenha_id())
        .bind(vaga.obtenha_coordenador().obtenha_usuario().obtenha_id())
        .bind(
            vaga.obtenha_vice_coordenador()
                .map(|vice| vice.obtenha_usuario().obtenha_id()),
        )
        .bind(vaga.obtenha_horas_por_semana() as i32)
        .bind(vaga.obtenha_imagem())
        .bind(vaga.obtenha_quantidade() as i32)
        .bind(vaga.obtenha_link_edital())
        .bind(vaga.obtenha_link_candidatura())
        .bind(vaga.obtenha_titulo())
        .bind(vaga.obtenha_conteudo())
        .bind(vaga.obtenha_data_de_inicio())
        .bind(vaga.obtenha_data_final_inscricoes())
        .bind(vaga.obtenha_data_de_cancelamento())
        .bind(vaga.obtenha_data_de_modificacao())
        .execute(self.db_conn)
        .await
        .map_err(|erro| {
            log::error!("Houve um erro inesperado no banco de dados: {erro}");
            ErroDeDominio::interno()
        })?;

        Ok(())
    }

    async fn buscar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Vaga>> {
        // TODO: transformar tudo num JOIN
        let vaga: Option<(
            Uuid,
            i32,
            String,
            i32,
            String,
            Option<String>,
            String,
            String,
            NaiveDate,
            NaiveDateTime,
            Option<NaiveDateTime>,
            Option<NaiveDateTime>,
            Uuid,
            Uuid,
            Option<Uuid>,
        )> = query_as(
            "
            SELECT \
                id, horas_por_semana, imagem, quantidade, link_edital, link_candidatura, \
                titulo, conteudo, iniciada_em, inscricoes_ate, cancelada_em, atualizada_em,
                id_projeto, id_coordenador, id_vice_coordenador \
            FROM vaga WHERE id = $1
        ",
        )
        .bind(id)
        .fetch_optional(self.db_conn)
        .await
        .map_err(|erro| {
            log::error!("{erro}");
            ErroDeDominio::interno()
        })?;

        let (
            id,
            horas_por_semana,
            imagem,
            quantidade,
            link_edital,
            link_candidatura,
            titulo,
            conteudo,
            iniciada_em,
            inscricoes_ate,
            cancelada_em,
            atualizada_em,
            id_projeto,
            id_coord,
            id_vice,
        ) = match vaga {
            None => return Ok(None),
            Some(vaga) => vaga,
        };

        let projeto = query_as("SELECT * FROM projeto WHERE id = $1")
            .bind(id_projeto)
            .fetch_one(self.db_conn)
            .await
            .map_err(|err| {
                log::error!("{err}");
                ErroDeDominio::interno()
            })?;

        let coordenador = query_as("SELECT * FROM usuario WHERE id = $1")
            .bind(id_coord)
            .fetch_one(self.db_conn)
            .await
            .map_err(|err| {
                log::error!("{err}");
                ErroDeDominio::interno()
            })?;

        let mut vaga = Vaga::criar_de_existente(
            id,
            projeto,
            coordenador,
            None,
            horas_por_semana,
            Some(imagem),
            quantidade,
            link_edital,
            link_candidatura,
            titulo,
            conteudo,
            iniciada_em,
            inscricoes_ate,
            cancelada_em,
            atualizada_em,
        );

        let vice = match id_vice {
            None => return Ok(Some(vaga)),
            Some(id) => query_as("SELECT * FROM usuario WHERE id = $1").bind(id),
        }
        .fetch_one(self.db_conn)
        .await
        .map_err(|err| {
            log::error!("{err}");
            ErroDeDominio::interno()
        })?;

        vaga.coloque_vice_coordenador(vice);

        Ok(Some(vaga))
    }

    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()> { todo!() }
}
