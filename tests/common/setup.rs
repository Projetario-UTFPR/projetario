use config::app::AppConfig;
use rstest::fixture;

#[fixture]
pub fn __setup() -> () {
    dotenvy::from_filename(".env.test")
        .expect("Failed to inject `.env` into environment variables.");

    let _ = env_logger::builder()
        .parse_env("RUST_LOG")
        .target(env_logger::Target::Stdout)
        .try_init();

    AppConfig::initialize();
}
