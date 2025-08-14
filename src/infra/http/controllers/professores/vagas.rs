use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, web};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
use crate::dominio::projetos::repositorios::projetos::RepositorioDeProjetos;
use crate::dominio::vagas::servicos::criar_vaga::CriarVagaParams;
use crate::infra::dtos::vagas::criar_vaga::CriarVagaDto;
use crate::infra::fabricas::servicos::criar_vaga::obtenha_servico_criar_vaga;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::{RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;
use crate::infra::repositorios::sqlx::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosSQLX;
use crate::infra::repositorios::sqlx::projetos::RepositorioDeProjetosSQLX;
use crate::infra::repositorios::sqlx::usuarios::RepositorioDeUsuariosSQLX;
use crate::unwrap_or_redirect;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio, TipoErroDeDominio};

pub struct ControllerVagas;
impl RouterRegistrable for ControllerVagas {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/vagas")
                .route("/nova", web::get().to(Self::nova))
                .route("/criar", web::post().to(Self::criar)),
        );
    }
}

impl ControllerVagas {
    pub async fn nova(req: HttpRequest) -> RespostaDoApp {
        Inertia::render(&req, "professores/vagas/nova".into())
            .await
            .map_err(Into::into)
    }

    pub async fn criar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        body: Json<CriarVagaDto>,
        usuario: UsuarioDaRequisicao,
    ) -> RedirectDoApp {
        let body = unwrap_or_redirect!(body.validate_or_back(&req));

        let criar_vaga = obtenha_servico_criar_vaga(&db_conn);

        let professor = match usuario {
            UsuarioDaRequisicao::Professor(prof) => Some(prof),
            _ => None,
        }
        .ok_or_else(|| {
            let errors = hashmap!["erro" => "Somente professores podem criar vagas.".into()];
            Inertia::back_with_errors(&req, errors)
        });

        let professor = unwrap_or_redirect!(professor);

        let params = CriarVagaParams {
            professor: &professor,
            id_projeto: body.id_projeto.unwrap(),
            horas_por_semana: body.horas_por_semana.unwrap(),
            imagem: body.imagem.unwrap(),
            quantidade: body.quantidade.unwrap(),
            link_edital: body.link_edital.unwrap(),
            conteudo: body.conteudo.unwrap(),
            titulo: body.titulo,
            link_candidatura: body.link_candidatura,
            inscricoes_ate: body.inscricoes_ate.unwrap().naive_utc(),
        };

        if let Err(erro) = criar_vaga.executar(params).await {
            return Inertia::back_with_errors(&req, hashmap!["erro" => erro.mensagem().into()]);
        }

        Inertia::back(&req)
    }
}
