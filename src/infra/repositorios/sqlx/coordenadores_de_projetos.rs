use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use futures_util::FutureExt;
use sqlx::postgres::{PgPoolCopyExt, PgRow};
use sqlx::{AnyPool, Connection, Executor, PgPool, Pool, Postgres, QueryBuilder, Row};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::{
    DirecaoOrdenacao,
    Filtro,
    Ordenador,
    Paginacao,
    ProjetosPaginados,
    RepositorioDeCoordenadoresDeProjetos,
    Tipo,
};
use crate::utils::erros::ResultadoDominio;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
pub struct RepositorioDeCoordenadoresDeProjetosSQLX<'this> {
    db_conn: &'this PgPool,
}

impl<'a> RepositorioDeCoordenadoresDeProjetosSQLX<'a> {
    pub fn novo(pool: &'a PgPool) -> Self { Self { db_conn: pool } }
}

#[async_trait]
impl RepositorioDeCoordenadoresDeProjetos for RepositorioDeCoordenadoresDeProjetosSQLX<'_> {
    async fn criar_projeto_com_coordenador(
        &self,
        projeto: &Projeto,
        coordenador: &Professor,
    ) -> Result<(), ErroDeDominio> {
        let criar_projeto = sqlx::query(
            "INSERT INTO \"projeto\" \
            (id, titulo, descricao, tipo, registrado_em, iniciado_em, atualizado_em, cancelado_em, concluido_em) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(projeto.obtenha_id())
        .bind(projeto.obtenha_titulo())
        .bind(projeto.obtenha_descricao())
        .bind(projeto.obtenha_tipo())
        .bind(projeto.obtenha_data_de_registro())
        .bind(projeto.obtenha_data_de_inicio())
        .bind(projeto.obtenha_data_de_modificacao())
        .bind(projeto.obtenha_data_de_cancelamento())
        .bind(projeto.obtenha_data_de_conclusao());

        let associar_professor_como_coordenador = sqlx::query(
            "INSERT INTO \"coordenador_projeto\" \
            (id_coordenador, id_projeto, tipo) \
            VALUES ($1, $2, $3)",
        )
        .bind(coordenador.obtenha_usuario().obtenha_id())
        .bind(projeto.obtenha_id())
        .bind(TipoDeCoordenacao::Coordenador);

        let mut transaction = self.db_conn.begin().await.map_err(|err| {
            log::error!("Não foi possível iniciar uma transação no banco de dados: {err}");
            ErroDeDominio::interno()
        })?;

        criar_projeto
            .execute(&mut *transaction)
            .await
            .map_err(|erro| {
                // Não tem nenhum erro que possa acontecer por falta de integridade na tabela "projeto"
                // Então, assume-se que foi um problema interno.
                log::warn!(
                    "Houve um erro, possívelmente inesperado, na tabela \"projeto\": {erro}"
                );

                ErroDeDominio::interno()
            })?;

        associar_professor_como_coordenador
            .execute(&mut *transaction)
            .await
            .map_err(|err| {
                let db_err = match err.as_database_error() {
                    Some(err) => err,
                    None => {
                        log::error!(
                            "Houve um erro, possívelmente inesperado, no banco de dados: {err}"
                        );
                        return ErroDeDominio::interno();
                    }
                };

                let err_code = match db_err.code() {
                    None => return ErroDeDominio::interno(),
                    Some(code) => code,
                };

                if err_code.as_ref() == "23503" {
                    return ErroDeDominio::integridade(
                        "Não foi encontrado nenhum professor vinculado a esse ID.",
                    );
                }

                log::warn!("Houve um erro não tratado no banco de dados: {err}");
                ErroDeDominio::interno()
            })?;

        transaction.commit().await.map_err(|err| {
            log::error!("Não foi possível comitar uma transação: {err}");
            ErroDeDominio::interno()
        })?;

        Ok(())
    }

    async fn buscar_projetos(
        &self,
        filtro: Filtro,
        tipo: Option<Tipo>,
        ordenador: Ordenador,
        paginacao: Paginacao,
    ) -> Result<ProjetosPaginados, ErroDeDominio> {
        let mut busca = QueryBuilder::<Postgres>::new(
            r#"SELECT id, titulo, descricao, tipo, registrado_em, iniciado_em, atualizado_em, cancelado_em, concluido_em FROM projeto"#,
        );

        let mut tem_condicoes = false;

        match filtro {
            Filtro::Titulo(titulo) => {
                busca.push(" WHERE titulo ILIKE '%' || ");
                busca.push_bind(titulo);
                busca.push(" || '%'");
                tem_condicoes = true;
            }
        }

        if let Some(Tipo::Tipo(tipo)) = tipo {
            if tem_condicoes {
                busca.push(" AND tipo = ");
            } else {
                busca.push(" WHERE tipo = ");
                tem_condicoes = true;
            }
            busca.push_bind(tipo);
        }

        match ordenador {
            Ordenador::Data(ordem) => {
                busca.push(" ORDER BY iniciado_em ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
                };
            }
            Ordenador::Titulo(ordem) => {
                busca.push(" ORDER BY titulo ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
                };
            }
        };

        let limite = (paginacao.pagina - 1) * paginacao.qtd_por_pagina as u32;
        busca
            .push(" LIMIT ")
            .push_bind(paginacao.qtd_por_pagina as i32)
            .push(" OFFSET ")
            .push_bind(limite as i32);

        let projetos = busca
            .build_query_as::<Projeto>()
            .fetch_all(self.db_conn)
            .await
            .map_err(|err| {
                log::error!("Falha ao buscar projetos: {}", err);
                ErroDeDominio::interno()
            })?;

        Ok(ProjetosPaginados {
            projetos,
            qtd_por_pagina: paginacao.qtd_por_pagina,
        })
    }

    // TODO: reescrever isso aqui como um join
    async fn buscar_coordenadores_do_projeto(
        &self,
        projeto: &Projeto,
    ) -> ResultadoDominio<(Professor, Option<Professor>)> {
        let coord = sqlx::query_as(
            "SELECT * FROM usuario WHERE id = (\
                SELECT id_coordenador FROM coordenador_projeto WHERE tipo = 'coordenador' AND id_projeto = $1\
            )"
        ).bind(projeto.obtenha_id()).fetch_one(self.db_conn);

        let vice_coord = sqlx::query_as(
            "SELECT * FROM usuario WHERE id = (\
                SELECT id_coordenador FROM coordenador_projeto WHERE tipo = 'vice_coordenador' AND id_projeto = $1\
            )"
        ).bind(projeto.obtenha_id()).fetch_optional(self.db_conn);

        tokio::try_join!(coord, vice_coord).map_err(|err| {
            log::error!("{err}");
            ErroDeDominio::interno()
        })
    }
}
