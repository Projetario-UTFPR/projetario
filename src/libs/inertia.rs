use inertia_rust::InertiaError;

use crate::utils::erros::erro_de_dominio::ErroDeDominio;

impl From<InertiaError> for ErroDeDominio {
    fn from(val: InertiaError) -> Self {
        log::error!("Renderização de uma resposta do Inertia resultou em erro: {val}");
        ErroDeDominio::interno()
    }
}
