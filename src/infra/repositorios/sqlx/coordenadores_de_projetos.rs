use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use futures_util::FutureExt;
use sqlx::postgres::PgPoolCopyExt;
use sqlx::{AnyPool, Connection, Executor, PgPool, Pool};

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
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
}
