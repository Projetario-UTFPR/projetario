use async_trait::async_trait;
use comum::erros::{ErroDeDominio, ResultadoDominio};
use dominio::comum::filtragem::DirecaoOrdenacao;
use dominio::comum::paginacao::{EntidadePaginada, Paginacao};
use dominio::projetos::entidades::projeto::Projeto;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::projetos::filtragem::{EstadoDoProjeto, FiltroDeProjeto, OrdenacaoDeProjeto};
use dominio::projetos::repositorios::projetos::RepositorioDeProjetos;
use sqlx::{PgPool, Postgres, QueryBuilder, query_as};
use uuid::Uuid;

pub struct RepositorioDeProjetosSQLX<'this> {
    db_conn: &'this PgPool,
}

impl<'this> RepositorioDeProjetosSQLX<'this> {
    pub fn novo(db_conn: &'this PgPool) -> Self { Self { db_conn } }
}

#[async_trait]
impl RepositorioDeProjetos for RepositorioDeProjetosSQLX<'_> {
    async fn encontrar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Projeto>> {
        query_as("SELECT * FROM projeto WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_conn)
            .await
            .map_err(|err| {
                log::error!("{err}");
                ErroDeDominio::interno()
            })
    }

    async fn buscar_projetos(
        &self,
        filtro: Option<FiltroDeProjeto>,
        estado: Option<EstadoDoProjeto>,
        tipo: Option<TipoDeProjeto>,
        ordenador: OrdenacaoDeProjeto,
        paginacao: Paginacao,
    ) -> Result<EntidadePaginada<Projeto>, ErroDeDominio> {
        let mut busca = QueryBuilder::<Postgres>::new(
            "SELECT
                proj.id, \
                proj.titulo, \
                proj.descricao, \
                proj.tipo, \
                proj.registrado_em, \
                proj.iniciado_em, \
                proj.atualizado_em, \
                proj.cancelado_em, \
                proj.concluido_em \
            FROM projeto proj",
        );

        let mut count =
            QueryBuilder::<Postgres>::new("SELECT COUNT(proj.id) count FROM projeto proj");

        let mut tem_condicoes = false;

        if let Some(filtro) = &filtro {
            [&mut count, &mut busca].into_iter().for_each(|query| {
                match filtro {
                    FiltroDeProjeto::Titulo(titulo) => {
                        query
                            .push(" WHERE proj.titulo ILIKE '%' || ")
                            .push_bind(titulo.clone())
                            .push(" || '%'");
                    }
                    FiltroDeProjeto::Coordenacao(id_do_coordenador) => {
                        query
                            .push(
                                " JOIN coordenador_projeto coor ON coor.id_projeto = proj.id \
                                WHERE coor.id_coordenador = ",
                            )
                            .push_bind(id_do_coordenador);
                    }
                };
            });

            tem_condicoes = true;
        }

        if let Some(estado) = estado {
            [&mut count, &mut busca]
                .into_iter()
                .for_each(|query| match estado {
                    EstadoDoProjeto::Ativo => {
                        if tem_condicoes {
                            query.push(" AND proj.cancelado_em IS NULL");
                        } else {
                            query.push(" WHERE proj.cancelado_em IS NULL");
                        }
                        query.push(" AND proj.concluido_em IS NULL");
                    }
                    EstadoDoProjeto::Cancelado => {
                        if tem_condicoes {
                            query.push(" AND proj.cancelado_em IS NOT NULL");
                        } else {
                            query.push(" WHERE proj.cancelado_em IS NOT NULL");
                        }
                    }
                    EstadoDoProjeto::Concluido => {
                        if tem_condicoes {
                            query.push(" AND proj.concluido_em IS NOT NULL");
                        } else {
                            query.push(" WHERE proj.concluido_em IS NOT NULL");
                        }
                    }
                });

            tem_condicoes = true;
        }

        if let Some(tipo) = tipo {
            if tem_condicoes {
                busca.push(" AND proj.tipo = ");
                count.push(" AND proj.tipo = ");
            } else {
                busca.push(" WHERE proj.tipo = ");
                count.push(" WHERE proj.tipo = ");
            }

            // tem_condicoes = true;
            busca.push_bind(tipo);
            count.push_bind(tipo);
        }

        match &ordenador {
            OrdenacaoDeProjeto::Data(ordem) => {
                busca.push(" ORDER BY proj.iniciado_em ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
                };
            }
            OrdenacaoDeProjeto::Titulo(ordem) => {
                busca.push(" ORDER BY proj.titulo ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
                };
            }
        };

        let offset = (paginacao.pagina - 1) * paginacao.qtd_por_pagina as u64;
        busca
            .push(" LIMIT ")
            .push_bind(paginacao.qtd_por_pagina as i32)
            .push(" OFFSET ")
            .push_bind(offset as i64);

        let (projetos, qtd_total): (_, i64) = tokio::try_join!(
            busca.build_query_as::<Projeto>().fetch_all(self.db_conn),
            count.build_query_scalar().fetch_one(self.db_conn)
        )
        .map_err(|err| {
            log::error!("{err}");
            ErroDeDominio::interno()
        })?;

        Ok(EntidadePaginada {
            dados: projetos,
            qtd_total: qtd_total as u64,
        })
    }
}
