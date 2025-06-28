use actix_web::cookie::Cookie;
use actix_web::dev::ServiceResponse;
use actix_web::http::header;
use config::app::AppConfig;

pub fn extraia_cookie_da_sessao(response: &ServiceResponse) -> Cookie<'_> {
    response
        .response()
        .cookies()
        .find(|cookie| cookie.name() == AppConfig::get().sessions_cookie_name)
        .expect("Response should contain a session cookie.")
}

pub fn extraia_valor_do_header_location(response: &ServiceResponse) -> &str {
    response
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap()
}
