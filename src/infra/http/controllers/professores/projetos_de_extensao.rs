use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, web};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use sqlx::PgPool;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::traits::IntoUsuarioModelo;
use crate::dominio::projetos::servicos::criar_projeto_de_extensao::{
    CriarProjetosDeExtensaoParams,
    ServicoCriarProjetoDeExtensao,
};
use crate::infra::dtos::projetos::criar_projeto::CriarProjetoDto;
use crate::infra::fabricas::servicos::criar_projeto_de_extensao::obtenha_servico_criar_projeto_de_extensao;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;
use crate::unwrap_or_redirect;

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
        Inertia::render(&req, "professores/projetos/novo-projeto-de-extensao".into())
            .await
            .map_err(Into::into)
    }

    // rota para persistir o novo projeto no banco de dados
    pub async fn criar_e_associar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        body: Json<CriarProjetoDto>,
        usuario: UsuarioDaRequisicao,
    ) -> RedirectDoApp {
        let body = unwrap_or_redirect!(body.validate_or_back(&req));
        let servico = obtenha_servico_criar_projeto_de_extensao(&db_conn);

        let professor = match usuario {
            UsuarioDaRequisicao::Professor(professor) => professor,
            _ => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Somente um professor ou um administrador pode criar um novo projeto de extensão.".into()],
                );
            }
        };

        if let Err(erro) = servico
            .executar(CriarProjetosDeExtensaoParams {
                data_de_inicio: body.data_de_inicio,
                descricao: body.descricao.unwrap(),
                titulo: body.titulo.unwrap(),
                professor: &professor,
            })
            .await
        {
            return Inertia::back_with_errors(&req, hashmap!["erro" => erro.mensagem().into()]);
        };

        Inertia::back(&req)
    }
}
