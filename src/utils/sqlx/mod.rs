use config::app::{AppConfig, RustEnv};
use sqlx::{
    ConnectOptions, Connection, Database, PgConnection, PgPool,
    any::AnyConnectOptions,
    migrate::Migrator,
    postgres::{PgConnectOptions, PgPoolOptions},
};
use std::{str::FromStr, time::Duration};

pub async fn migrate_db(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!().run(pool).await;
    Ok(())
}

pub async fn connect_to_db() -> anyhow::Result<PgPool> {
    let mut db_opts = PgConnectOptions::from_str(AppConfig::get().main_database_url)?;

    if AppConfig::get().environment == RustEnv::Production {
        db_opts = db_opts.disable_statement_logging();
    }

    let mut db_pool_opts = PgPoolOptions::new()
        .max_connections(AppConfig::get().main_database_connections)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    let connection = db_pool_opts.connect_with(db_opts).await?;

    Ok(connection)
}
