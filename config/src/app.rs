use std::env::var;
use std::sync::OnceLock;

use base64::Engine;
use base64::engine::general_purpose;

static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

#[derive(Default, PartialEq, Eq, Debug)]
pub enum RustEnv {
    #[default]
    Development,
    Production,
    Test,
}

pub struct AppConfig {
    // program settings
    pub app_name: &'static str,
    pub app_url: &'static str,
    pub app_key: &'static [u8],
    pub environment: RustEnv,

    // infra settings
    pub main_database_schema: Option<&'static str>,
    pub main_database_url: &'static str,
    pub main_database_connections: u32,

    // sessions settings
    pub sessions_flash_key: &'static str,
    pub sessions_user_key: &'static str,
    pub sessions_errors_key: &'static str,
    pub sessions_prev_req_key: &'static str,
    pub sessios_curr_req_key: &'static str,
    pub sessions_dir: &'static str,
    pub sessions_exp_key: &'static str,
    pub sessions_cookie_name: &'static str,
    pub garbage_collector_lottery: [u8; 2],
}

impl AppConfig {
    fn get_app_options() -> Self {
        let app_name = Box::leak(
            var("APP_NAME")
                .unwrap_or("Projetário UTFPR".into())
                .into_boxed_str(),
        );

        Self {
            app_name,
            app_url: Box::leak(
                var("APP_URL")
                    .expect("Environment variables should contain `APP_URL`.")
                    .into_boxed_str(),
            ),
            main_database_url: Box::leak(
                var("MAIN_DATABASE_URL")
                    .expect("Environment variables should contain `MAIN_DATABASE_URL`.")
                    .into_boxed_str(),
            ),
            main_database_schema: var("MAIN_DB_SCHEMA")
                .ok()
                .map(|schema| Box::leak(schema.into_boxed_str()) as &'static str),
            main_database_connections: var("MAIN_DATABASE_CONNECTIONS")
                .expect("Environment variables should contain a valid `MAIN_DATABASE_CONNECTIONS`.")
                .parse()
                .expect("`MAIN_DATABASE_CONNECTIONS` value should be a unsigned 32 bits integer."),
            environment: var("RUST_ENV").map(Into::into).unwrap_or_default(),
            app_key: var("APP_KEY")
                .map(|key| {
                    general_purpose::STANDARD
                        .decode(&key)
                        .expect("APP_KEY should be a 64bytes base64-encoded string.")
                        .leak()
                })
                .expect("Environment variables should contan `APP_KEY`."),

            sessions_dir: "storage/sessions",
            sessions_cookie_name: format_session_cookie_from_app_name(app_name),
            sessions_exp_key: "__expires_at__",
            garbage_collector_lottery: [2, 100],
            sessions_flash_key: "__flash__",
            sessions_user_key: "__user__",
            sessions_errors_key: "__errors__",
            sessions_prev_req_key: "__prev_req_url__",
            sessios_curr_req_key: "__curr_req_url__",
        }
    }

    pub fn initialize() { APP_CONFIG.get_or_init(Self::get_app_options); }

    pub fn get() -> &'static Self { APP_CONFIG.get_or_init(Self::get_app_options) }
}

impl<T: ToString> From<T> for RustEnv {
    fn from(value: T) -> Self {
        match value.to_string().as_str() {
            "DEVELOPMENT" | "development" => RustEnv::Development,
            "PRODUCTION" | "production" => RustEnv::Production,
            "TEST" | "test" => RustEnv::Test,
            _ => panic!("Invalid RustEnv value"),
        }
    }
}

fn format_session_cookie_from_app_name(app_name: &str) -> &'static str {
    let app_name = app_name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .to_ascii_lowercase()
        .replace(' ', "_");
    let session_cookie_name = format!("{app_name}_session").into_boxed_str();
    Box::leak(session_cookie_name)
}
