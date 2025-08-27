use actix_web::web::Json;
use actix_web::{HttpRequest, Responder, web};
use inertia_rust::{Inertia, InertiaFacade};

use crate::infra::dtos::tema::TemaEstritoDto;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::RespostaDoApp;
use crate::infra::tema::GerenteDeTema;

pub struct TemaController;

impl RouterRegistrable for TemaController {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("tema")
                .route("proximo", web::post().to(Self::alterar_tema))
                .route("sistema", web::post().to(Self::alterar_tema_do_sistema)),
        );
    }
}

impl TemaController {
    pub async fn alterar_tema(req: HttpRequest, gerente_de_tema: GerenteDeTema) -> RespostaDoApp {
        let mut res = Inertia::back(&req).respond_to(&req).map_into_boxed_body();

        gerente_de_tema.persista_tema_silenciosamente(
            &mut res,
            gerente_de_tema
                .extraia_tema(&req)
                .unwrap_or_default()
                .seguinte(),
        );

        Ok(res)
    }

    pub async fn alterar_tema_do_sistema(
        req: HttpRequest,
        gerente_de_tema: GerenteDeTema,
        body: Json<TemaEstritoDto>,
    ) -> RespostaDoApp {
        let mut res = Inertia::back(&req).respond_to(&req).map_into_boxed_body();
        gerente_de_tema.persista_tema_do_sistema_silenciosamente(&mut res, body.into_inner().tema);
        Ok(res)
    }
}
