use std::str::FromStr;
use std::time::Duration;

use config::app::{AppConfig, RustEnv};
use sqlx::any::AnyConnectOptions;
use sqlx::migrate::Migrator;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{ConnectOptions, Connection, Database, PgConnection, PgPool};

pub async fn migrate_db(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await;
    Ok(())
}

pub async fn connect_to_db_from_app_config() -> anyhow::Result<PgPool> {
    connect_to_db(AppConfig::get().main_database_url, None).await
}

pub async fn connect_to_db(
    database_url: &'static str,
    schema: Option<&'static str>,
) -> anyhow::Result<PgPool> {
    let mut db_opts = PgConnectOptions::from_str(database_url)?;

    if AppConfig::get().environment == RustEnv::Production {
        db_opts = db_opts.disable_statement_logging();
    }

    let mut db_pool_opts = PgPoolOptions::new()
        .max_connections(AppConfig::get().main_database_connections)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .after_connect(move |db_conn, _meta| {
            Box::pin(async move {
                if let Some(schema) = schema {
                    sqlx::query(&format!("SET search_path = {}", schema))
                        .execute(db_conn)
                        .await?;
                }

                Ok(())
            })
        });

    let connection = db_pool_opts.connect_with(db_opts).await?;

    Ok(connection)
}
