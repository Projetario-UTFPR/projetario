#![allow(unused)]

use actix_web::HttpServer;
use actix_web::web::Data;
use config::app::{AppConfig, RustEnv};
use config::inertia::get_inertia;
use config::vite::get_vite;
use env_logger::Target;
use libs::actix::server::get_server;

use crate::utils::resolve_uri;
use crate::utils::sqlx::{connect_to_db, migrate_db};

mod dominio;
mod infra;
mod libs;
mod utils;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().expect("Não foi possível inicializar as variáveis de ambiente.");

    env_logger::builder()
        .parse_env("RUST_LOG")
        .target(Target::Stdout)
        .init();

    let db_pool = connect_to_db().await?;
    migrate_db(&db_pool).await?;

    let app_config = AppConfig::get();

    let is_production_env = app_config.environment == RustEnv::Production;
    let mut ssr_server_process = None;

    let vite = get_vite().await?;
    let inertia = Data::new(get_inertia(vite).await?);
    let inertia_data = inertia.clone();

    let (host, port) = resolve_uri(is_production_env);

    let http_server =
        HttpServer::new(move || get_server().app_data(inertia_data.clone())).bind((host, port))?;

    if is_production_env {
        ssr_server_process = inertia
            .start_node_server("dist/ssr/ssr.js".to_string())?
            .into();
    }

    log::info!("Iniciando o servidor em {host}:{port}.");
    let signal = http_server.run().await;

    if let Some(process) = ssr_server_process {
        let _ = process.kill().await;
    }

    signal?;

    Ok(())
}
