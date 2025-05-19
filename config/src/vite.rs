use vite_rust::{Vite, ViteConfig};

use super::app::{AppConfig, RustEnv};

pub async fn get_vite() -> anyhow::Result<Vite> {
    let mut vite_config = ViteConfig::default()
        .set_manifest_path("public/bundle/manifest.json")
        .set_entrypoints(vec!["www/app.tsx"])
        .set_prefix("bundle");

    if AppConfig::get().environment != RustEnv::Development {
        vite_config = vite_config.set_force_mode(vite_rust::ViteMode::Manifest);
    }

    Ok(Vite::new(vite_config).await?)
}
