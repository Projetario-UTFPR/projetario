use actix_web::web::Data;
use config::app::AppConfig;
use projetario::utils::sqlx::{connect_to_db, migrate_db};
use rstest::fixture;
use sqlx::PgPool;

use crate::common::utils::esquema_db::obtenha_esquema_unico_do_db;

#[fixture]
pub async fn db_guard<'a>(
    #[default(obtenha_esquema_unico_do_db().leak())] schema: &'static str,
) -> DBGuard<'a> {
    DBGuard::novo(schema).await
}

pub struct DBGuard<'this> {
    pub db_conn: Option<Data<PgPool>>,
    schema: &'this str,
}

impl DBGuard<'_> {
    pub async fn novo(schema: &'static str) -> Self {
        let options = AppConfig::get();

        let db_conn = connect_to_db(options.main_database_url, Some(schema))
            .await
            .expect("Failed to initialize datastore in test fixture.");

        if let Err(err) = sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema))
            .execute(&db_conn)
            .await
        {
            log::error!("{err}");
        };

        migrate_db(&db_conn).await.unwrap_or_else(|err| {
            panic!("Failed to run migration on test setup: {err}");
        });

        Self {
            db_conn: Some(Data::new(db_conn)),
            schema,
        }
    }

    /// Extrai a conexão do DBGuard, deixando um `None` em seu lugar.
    ///
    /// ## Panic
    /// Essa função resulta em pânico se a conexão já tiver sido retirada.
    #[allow(dead_code)]
    pub fn extraia_conexao(&mut self) -> Data<PgPool> {
        if self.db_conn.is_none() {
            panic!(
                "Tentou extrair a conexão do banco de dados de um DBGuard que já tivera a conexão extraída."
            );
        }

        self.db_conn.take().unwrap()
    }

    /// Clona a conexão deste DBGuard.
    ///
    /// ## Panic
    /// Essa função resulta em pânico se a conexão tiver sido removida anteriormente.
    pub fn clone_a_conexao(&self) -> Data<PgPool> { self.db_conn.as_ref().unwrap().clone() }
}

impl AsRef<PgPool> for DBGuard<'_> {
    fn as_ref(&self) -> &PgPool { self.db_conn.as_ref().unwrap() }
}

impl Drop for DBGuard<'_> {
    fn drop(&mut self) {
        let schema = self.schema.to_owned();
        let database_url = AppConfig::get().main_database_url.to_owned();

        let previous_datasotre = self.db_conn.clone();

        let _ = std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Couldn't initialize tokio runtime to drop database test schema.")
                .block_on(async move {
                    if let Some(datastore) = previous_datasotre {
                        datastore.close().await;
                    }

                    let database_url = database_url.leak();
                    let schema = schema.leak();
                    let datastore = connect_to_db(database_url, Some(schema)).await.unwrap();

                    if let Err(err) =
                        sqlx::query(&format!("DROP SCHEMA IF EXISTS {} CASCADE", schema))
                            .execute(&datastore)
                            .await
                    {
                        log::error!("{err}");
                    };
                })
        })
        .join();
    }
}
