use config::app::AppConfig;
use rstest::fixture;

#[fixture]
pub fn loaded_options<'a>() -> &'a AppConfig {
    dotenvy::from_filename(".env.test")
        .expect("Could not to load .env.test file into environment variables.");

    AppConfig::get()
}
