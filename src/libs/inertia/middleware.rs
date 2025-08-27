use std::sync::Arc;

use actix_session::SessionExt;
use actix_web::HttpMessage;
use config::app::AppConfig;
use dominio::identidade::traits::IntoUsuarioModelo;
use futures_util::FutureExt;
use inertia_rust::actix::InertiaMiddleware;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap};
use serde_json::Map;

use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;
use crate::infra::http::presenters::usuario_modelo::UsuarioModeloPresenter;
use crate::infra::tema::GerenteDeTema;

pub fn get_inertia_middleware<'a>() -> InertiaMiddleware<'a> {
    InertiaMiddleware::new().with_shared_props(Arc::new(|req| {
        let app_config = AppConfig::get();

        let usuario = req.extensions().get::<UsuarioDaRequisicao>().unwrap_or_else(|| {
            log::warn!("Usuário da requisição não encontrada pelo `InertiaMiddleware`, caindo para o usuário convidado.");
            &UsuarioDaRequisicao::Convidado
        }).clone();

        let usuario = match usuario {
            UsuarioDaRequisicao::Convidado => None,
            UsuarioDaRequisicao::Aluno(aluno) => Some(aluno.into_usuario_modelo()),
            UsuarioDaRequisicao::Professor(professor) => Some(professor.into_usuario_modelo())
        };

        let autenticacao = usuario.map(|usuario| {
            hashmap!["usuario".to_string() => UsuarioModeloPresenter::apresente(&usuario)]
        });

        let flash = req
            .get_session()
            .remove(app_config.sessions_flash_key)
            .map(|flash_map| serde_json::from_str::<Map<_, _>>(&flash_map).unwrap_or_default())
            .unwrap_or_default();

        let gerente_de_tema = GerenteDeTema::empreste_da_requisicao(req);
        let tema = gerente_de_tema.extraia_tema(req).unwrap_or_default();
        let tema_do_sistema = gerente_de_tema.extraia_tema_do_sistema(req).unwrap_or_default();

        Inertia::view_data(req, hashmap![
            "tema" => tema.clone().to_string().into(),
            "tema_do_sistema" => tema_do_sistema.clone().to_string().into()
        ]);

        async { hashmap![
            "flash" => InertiaProp::always(flash),
            "autenticacao" => InertiaProp::data(autenticacao),
            "temaPreferido" => InertiaProp::data(tema),
            "temaSistema" => InertiaProp::data(tema_do_sistema)
        ] }.boxed_local()
    }))
}
