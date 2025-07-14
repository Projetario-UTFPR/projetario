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
            r#"INSERT INTO vaga (
                id, id_projeto, id_coordenador, id_vice_coordenador,
                horas_por_semana, imagem, quantidade, link_edital,
                link_candidatura, titulo, conteudo, iniciada_em,
                inscricoes_ate, cancelada_em, atualizada_em)
            VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12,
                $13, $14, $15
            )"#,
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
        sqlx::query_as(&format!("{SELECT_VAGA_QUERY} WHERE v.id = $1"))
            .bind(id)
            .fetch_optional(self.db_conn)
            .await
            .map_err(|err| {
                log::error!("{err}");
                ErroDeDominio::interno()
            })
    }

    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()> { todo!() }
}

const SELECT_VAGA_QUERY: &str = r#"SELECT
        v.*,

        -- projeto
        p.id as "p_id",
        p.titulo as "p_titulo",
        p.descricao as "p_descricao",
        p.tipo as "p_tipo",
        p.registrado_em as "p_registrado_em",
        p.iniciado_em as "p_iniciado_em",
        p.atualizado_em as "p_atualizado_em",
        p.cancelado_em as "p_cancelado_em",
        p.concluido_em as "p_concluido_em",

        -- coordenador
        c.id as "c_id",
        c.nome as "c_nome",
        c.email as "c_email",
        c.senha_hash as "c_senha_hash",
        c.url_curriculo_lattes as "c_url_curriculo_lattes",
        c.atualizado_em as "c_atualizado_em",
        c.desativado_em as "c_desativado_em",
        c.registrado_em as "c_registrado_em",

        -- vice coordenador
        vice.id as "vice_id",
        vice.nome as "vice_nome",
        vice.email as "vice_email",
        vice.senha_hash as "vice_senha_hash",
        vice.url_curriculo_lattes as "vice_url_curriculo_lattes",
        vice.atualizado_em as "vice_atualizado_em",
        vice.desativado_em as "vice_desativado_em",
        vice.registrado_em as "vice_registrado_em"
    FROM vaga v
    -- projeto
    INNER JOIN projeto p ON p.id = v.id_projeto

    -- coordenador
    INNER JOIN coordenador_projeto c_rel
        ON c_rel.id_projeto = p.id
        AND c_rel.tipo = 'coordenador'
    INNER JOIN usuario c ON c.id = c_rel.id_coordenador

    -- vice coordenador
    LEFT JOIN coordenador_projeto vice_rel
        ON vice_rel.id_projeto = p.id
        AND vice_rel.tipo = 'vice_coordenador'
    LEFT JOIN usuario vice ON vice.id = vice_rel.id_coordenador
    "#;
