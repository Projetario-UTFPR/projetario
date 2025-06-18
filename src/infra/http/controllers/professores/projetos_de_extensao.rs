use actix_web::web::Data;
use actix_web::{HttpRequest, web};
use inertia_rust::{Inertia, InertiaFacade};
use sqlx::PgPool;

use crate::dominio::projetos::servicos::criar_projeto_de_extensao::ServicoCriarProjetoDeExtensao;
use crate::infra::fabricas::servicos::criar_projeto_de_extensao::obtenha_servico_criar_projeto_de_extensao;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};

pub struct ControllerProjetosDeExtensao;

impl Controller for ControllerProjetosDeExtensao {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/projetos/extensao")
                .route("/novo", web::get().to(Self::novo))
                .route("/criar_e_associar", web::post().to(Self::criar_e_associar)),
        );
    }
}

impl ControllerProjetosDeExtensao {
    // página que exibe o formulário para criação de um novo projeto de extensão
    pub async fn novo(req: HttpRequest) -> RespostaDoApp {
        // TODO: criar essa página no front-end
        Inertia::render(&req, "professores/projetos/novo-projeto-de-extensao".into())
            .await
            .map_err(Into::into)
    }

    // rota para persistir o novo projeto no banco de dados
    pub async fn criar_e_associar(req: HttpRequest, db_conn: Data<PgPool>) -> RedirectDoApp {
        let servico = obtenha_servico_criar_projeto_de_extensao(&db_conn);

        // TODO: criar o dto e extraí-lo da requisição HTTP
        // TODO: validar os dados do formulário pelo DTO
        // TODO: extrair o professor da requisição HTTP (ele deve estar logado pra isso)
        // TODO: chamar o serviço pra criar e associar o projeto ao professor logado

        Inertia::back(&req)
    }
}
