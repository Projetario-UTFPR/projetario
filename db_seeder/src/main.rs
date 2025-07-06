use std::str::FromStr;
use std::time::Duration;

use config::app::AppConfig;
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use crate::usuarios::inserir_usuarios;

mod senhas;
mod usuarios;

#[tokio::main]
async fn main() {
    dotenv().expect("Não foi possível carregar as variáveis de ambiente. Abortando o DB Seeding.");
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stdout)
        .init();

    log::warn!(
        "Não utilize esse módulo em produção! Ele insere \
        tuplas no banco de dados para fins de testes em desenvolvimento."
    );

    let app_config = AppConfig::get();

    let db_pool = conectar_no_db(app_config.main_database_url).await;

    sqlx::migrate!("../migrations")
        .run(&db_pool)
        .await
        .expect("Não foi possível rodar as migrations do banco de dados durante o seed.");

    log::info!("Iniciando o seeding no banco de dados");
    inserir_usuarios(&db_pool).await;
}

async fn conectar_no_db(db_url: &str) -> PgPool {
    let db_opts = PgConnectOptions::from_str(db_url)
        .expect("O URL de conexão do banco de dados é inválido, e não será possível rodar o seed.");

    let db_pool_opts = PgPoolOptions::new()
        .max_connections(AppConfig::get().main_database_connections)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8));

    db_pool_opts
        .connect_with(db_opts)
        .await
        .expect("Não foi possível se conectar com o banco de dados para realizar o seed.")
}
