use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, web};
use comum::erros::ErroDeDominio;
use dominio::comum::paginacao::Paginacao;
use dominio::projetos::filtragem::{EstadoDoProjeto, FiltroDeProjeto};
use dominio::projetos::servicos::buscar_projetos_de_extensao::BuscarProjetosDeExtensaoParams;
use dominio::vagas::servicos::criar_vaga::CriarVagaParams;
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, InertiaProp, hashmap, prop_resolver};
use sqlx::PgPool;

use crate::infra::dtos::vagas::criar_vaga::CriarVagaDto;
use crate::infra::fabricas::servicos::buscar_projeto_de_extensao::obtenha_servico_buscar_de_projetos_de_extensao;
use crate::infra::fabricas::servicos::criar_vaga::obtenha_servico_criar_vaga;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::{RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;
use crate::infra::http::presenters::paginacao::PaginacaoMelhoradaPresenter;
use crate::infra::http::presenters::projeto::ProjetoPresenter;
use crate::libs::inertia::inertiafy_domain_error;
use crate::unwrap_or_redirect;

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
    pub async fn nova(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        usuario: UsuarioDaRequisicao,
    ) -> RespostaDoApp {
        let UsuarioDaRequisicao::Professor(professor) = usuario else {
            return Err(ErroDeDominio::nao_autorizado(
                "Somente um professor pode criar vagas para um projeto.",
            ));
        };
        let id_professor = professor.obtenha_usuario().obtenha_id().to_owned();

        Inertia::render_with_props(
            &req,
            "professores/vagas/nova".into(),
            hashmap![
                "projetos" => InertiaProp::Deferred(prop_resolver!(let db_conn = db_conn.clone(); {
                    let projetos = obtenha_servico_buscar_de_projetos_de_extensao(&db_conn)
                        .executar(BuscarProjetosDeExtensaoParams {
                            estado: Some(EstadoDoProjeto::Ativo),
                            filtro: Some(FiltroDeProjeto::Coordenacao(id_professor)),
                            ordenador: None,
                            paginacao: Paginacao::nova(1, 100),
                        })
                        .await
                        .map(|projetos_paginados| PaginacaoMelhoradaPresenter::apresente(&projetos_paginados, ProjetoPresenter::apresente));

                    inertiafy_domain_error(projetos)
                }), None)
            ],
        )
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
