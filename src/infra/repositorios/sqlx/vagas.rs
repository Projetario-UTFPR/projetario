use std::collections::HashSet;

use comum::erros::{ErroDeDominio, ResultadoDominio};
use const_format::concatcp;
use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::comum::filtragem::LimitadorDeData;
use dominio::comum::paginacao::{EntidadePaginada, Paginacao};
use dominio::vagas::entidades::vaga::Vaga;
use dominio::vagas::filtragem::{EstadoDaVaga, FiltroDeVaga, OrdenacaoDeVaga};
use dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

pub struct RepositorioDeVagasSQLX<'this> {
    db_conn: &'this PgPool,
}

impl<'this> RepositorioDeVagasSQLX<'this> {
    pub fn novo(db_conn: &'this PgPool) -> Self { Self { db_conn } }
}

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

    // TODO: implementar método de atualizar vaga no repositório SQLX
    async fn atualizar_vaga(&self, _vaga: &Vaga) -> ResultadoDominio<()> { todo!() }

    async fn buscar_vagas(
        &self,
        filtros: HashSet<FiltroDeVaga>,
        ordenador: OrdenacaoDeVaga,
        paginacao: Paginacao,
    ) -> ResultadoDominio<EntidadePaginada<Vaga>> {
        let mut busca = sqlx::QueryBuilder::<Postgres>::new(SELECT_VAGA_QUERY);
        let mut contagem = sqlx::QueryBuilder::<Postgres>::new(concatcp!(
            "SELECT COUNT(v.id) count ",
            SELECT_VAGA_JOINS
        ));

        busca.push(" WHERE TRUE");
        contagem.push(" WHERE TRUE");

        for filtro in &filtros {
            Self::transformar_filtro_em_sql(&mut busca, filtro);
            Self::transformar_filtro_em_sql(&mut contagem, filtro);
        }

        match ordenador {
            OrdenacaoDeVaga::Data(direcao) => busca
                .push(" ORDER BY v.iniciada_em ")
                .push(direcao.into_sql_string()),
            OrdenacaoDeVaga::Titulo(direcao) => busca
                .push(" ORDER BY v.titulo ")
                .push(direcao.into_sql_string()),
        };

        busca
            .push(" LIMIT ")
            .push_bind(paginacao.qtd_por_pagina as i32)
            .push(" OFFSET ")
            .push_bind(paginacao.calcule_offset() as i64);

        let (vagas, qtd_total): (_, i64) = tokio::try_join!(
            busca.build_query_as::<Vaga>().fetch_all(self.db_conn),
            contagem.build_query_scalar().fetch_one(self.db_conn)
        )
        .map_err(|err| {
            log::error!("{err}");
            ErroDeDominio::interno()
        })?;

        Ok(EntidadePaginada {
            dados: vagas,
            qtd_total: qtd_total as u64,
        })
    }
}

impl RepositorioDeVagasSQLX<'_> {
    fn transformar_filtro_em_sql<'a>(
        busca: &mut QueryBuilder<'a, Postgres>,
        filtro: &'a FiltroDeVaga,
    ) {
        match filtro {
            FiltroDeVaga::Titulo(titulo) => {
                busca
                    .push(" AND v.titulo ILIKE '%' || ")
                    .push_bind(titulo)
                    .push(" || '%'");
            }
            FiltroDeVaga::Coordenador(id) => {
                busca
                    .push(" AND (c.id = ")
                    .push_bind(id)
                    .push(" OR vice.id = ")
                    .push_bind(id)
                    .push(")");
            }
            FiltroDeVaga::Estado(estado) => {
                busca.push(" AND");
                let query_do_estado = match estado {
                    EstadoDaVaga::Ativa => " v.cancelada_em IS NULL AND v.inscricoes_ate > now()",
                    EstadoDaVaga::Cancelada => " v.cancelada_em IS NOT NULL",
                    EstadoDaVaga::Encerrada => " v.inscricoes_ate < now()",
                };

                busca.push(query_do_estado);
            }
            FiltroDeVaga::DataDePublicacao(data, limitador) => {
                busca
                    .push(" AND v.iniciada_em ")
                    .push(match limitador {
                        LimitadorDeData::Apos => "> ",
                        LimitadorDeData::Ate => "<= ",
                    })
                    .push_bind(data);
            }
            FiltroDeVaga::Tipo(tipo) => {
                busca.push(" AND p.tipo = ").push_bind(tipo);
            }
        }
    }
}

const SELECT_VAGA_QUERY: &str = concatcp!(
    r#"SELECT
        -- vaga
        v.id,
        v.id_projeto,
        v.id_coordenador,
        v.id_vice_coordenador,
        v.horas_por_semana,
        v.imagem,
        v.quantidade,
        v.link_edital,
        v.link_candidatura,
        v.titulo,
        v.conteudo,
        v.iniciada_em,
        v.inscricoes_ate,
        v.cancelada_em,
        v.atualizada_em,

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
    "#,
    SELECT_VAGA_JOINS
);

const SELECT_VAGA_JOINS: &str = r#"FROM vaga v
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
    LEFT JOIN usuario vice ON vice.id = vice_rel.id_coordenador"#;
