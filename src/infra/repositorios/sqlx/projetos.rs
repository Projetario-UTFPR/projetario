use async_trait::async_trait;
use sqlx::{PgPool, query_as};
use uuid::Uuid;

use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::repositorios::projetos::RepositorioDeProjetos;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};

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
}
