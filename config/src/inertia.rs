use inertia_rust::template_resolvers::ViteHBSTemplateResolver;
use inertia_rust::{Inertia, InertiaConfig, InertiaVersion};
use vite_rust::Vite;

use super::app::AppConfig;
use crate::app::RustEnv;

pub async fn get_inertia(vite: Vite) -> anyhow::Result<Inertia> {
    let assets_version =
        InertiaVersion::Literal(vite.get_hash().unwrap_or("DEVELOPMENT").to_owned());

    let template_resolver = ViteHBSTemplateResolver::builder()
        .set_vite(vite)
        .set_template_path("www/root.hbs")
        .build()?;

    let config = AppConfig::get();

    let mut inertia_config = InertiaConfig::builder()
        .set_version(assets_version)
        .set_template_resolver(Box::new(template_resolver))
        .set_url(config.app_url);

    if config.environment == RustEnv::Production {
        inertia_config = inertia_config.enable_ssr()
    }

    Ok(Inertia::new(inertia_config.build())?)
}
