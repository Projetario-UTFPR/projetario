use actix_web::web::{Data, Json};
use actix_web::{Either, HttpRequest, Responder, web};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use sqlx::PgPool;

use crate::dominio::projetos::repositorios::coordenadores_de_projetos::ProjetosPaginados;
use crate::dominio::projetos::servicos::buscar_projetos::{
    BuscarProjetosParams,
    ServicoBuscaProjetos,
};
use crate::infra::dtos::projetos::buscar_projeto::BuscarProjetoDto;
use crate::infra::fabricas::servicos::buscar_projetos::obtenha_servico_buscar_projetos;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};

pub struct ControllerProjetos;

impl Controller for ControllerProjetos {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(web::scope("/projetos").route("/pesquisar", web::get().to(Self::pesquisar)));
    }
}

impl ControllerProjetos {
    // página que exibe o resultado da pesquisa
    pub async fn novo(req: HttpRequest) -> RespostaDoApp {
        Inertia::render(&req, "geral/pesquisa".into())
            .await
            .map_err(Into::into)
    }

    // rota para pesquisar projetos
    pub async fn pesquisar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        body: Json<BuscarProjetoDto>,
    ) -> impl Responder {
        let body = match body.validate_or_back(&req) {
            Ok(body) => body,
            Err(redirect) => return Either::Left(redirect),
        };

        let servico = obtenha_servico_buscar_projetos(&db_conn);

        match servico
            .buscar_projeto(BuscarProjetosParams {
                filtro: body.filtro,
                tipo: body.tipo,
                ordenador: body.ordenador,
                pagina: body.pagina_atual,
            })
            .await
        {
            Ok(projetos) => Either::Right(actix_web::web::Json(projetos)),
            Err(erro) => Either::Left(Inertia::back_with_errors(
                &req,
                hashmap!["erro" => erro.mensagem().into()],
            )),
        }
    }
}
