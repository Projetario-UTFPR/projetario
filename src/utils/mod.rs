pub mod common;
pub mod erros;
pub mod sqlx;

#[cfg(feature = "test-utils")]
pub mod test;

pub fn resolve_uri<'b>(is_production_env: bool) -> (&'b str, u16) {
    // Se estiver sendo compilado em uma imagem Docker, é necessário que escute na porta padrão
    // no IP padrão, de modo que seja possível levantá-lo como um container docker e fazer o bind
    // dessa porta em outra porta (como está sendo feito no arquivo `compose.yaml`.)
    #[cfg(feature = "dockerimgb")]
    let must_listen_to_public_port = true;
    #[cfg(not(feature = "dockerimgb"))]
    let must_listen_to_public_port = is_production_env;

    let port = if must_listen_to_public_port { 80 } else { 3000 };

    let host = if must_listen_to_public_port {
        "0.0.0.0"
    } else {
        "127.0.0.1"
    };

    (host, port)
}
