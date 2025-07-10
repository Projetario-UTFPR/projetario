use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, web};
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use sqlx::PgPool;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::repositorios::projetos::RepositorioDeProjetos;
use crate::dominio::vagas::servicos::criar_vaga::CriarVagaParams;
use crate::infra::dtos::vagas::criar_vaga::CriarVagaDto;
use crate::infra::fabricas::servicos::criar_vaga::obtenha_servico_criar_vaga;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;
use crate::infra::repositorios::sqlx::projetos::RepositorioDeProjetosSQLX;
use crate::infra::repositorios::sqlx::usuarios::RepositorioDeUsuariosSQLX;
use crate::unwrap_or_redirect;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio, TipoErroDeDominio};

pub struct ControllerVagas;
impl Controller for ControllerVagas {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/vagas")
                .route("/nova", web::get().to(Self::nova))
                .route("criar", web::post().to(Self::criar)),
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
        let repositorio_de_projetos = RepositorioDeProjetosSQLX::novo(&db_conn);

        let professor = match usuario {
            UsuarioDaRequisicao::Professor(prof) => prof,
            _ => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Somente professores podem criar vagas.".into()],
                );
            }
        };

        let projeto = match repositorio_de_projetos
            .encontrar_por_id(&body.id_projeto)
            .await
        {
            Err(err) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Houve um problema no servidor.".into()],
                );
            }
            Ok(projeto) => projeto,
        };

        let projeto = match projeto {
            None => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "O projeto associado não foi encontrado.".into()],
                );
            }
            Some(projeto) => projeto,
        };

        let vice_coordenador =
            match busque_vice_coordenador_se_existir(body.id_vice_coordenador, &db_conn).await {
                Err(err) => {
                    return Inertia::back_with_errors(
                        &req,
                        hashmap!["error" => err.mensagem().into()],
                    );
                }
                Ok(coord) => coord,
            };

        let params = CriarVagaParams {
            projeto,
            coordenador: professor.clone(),
            vice_coordenador,
            horas_por_semana: body.horas_por_semana,
            imagem: body.imagem.clone(),
            quantidade: body.quantidade,
            link_edital: body.link_edital.clone(),
            conteudo: body.conteudo.clone(),
            titulo: body.titulo.clone(),
            link_candidatura: body.link_candidatura.clone(),
            inscricoes_ate: body.inscricoes_ate,
        };

        if let Err(erro) = criar_vaga.executar(params).await {
            return Inertia::back_with_errors(&req, hashmap!["erro" => erro.mensagem().into()]);
        }

        Inertia::back(&req)
    }
}

async fn busque_vice_coordenador_se_existir(
    id_vice: Option<Uuid>,
    db_conn: &PgPool,
) -> ResultadoDominio<Option<Professor>> {
    let id_vice = match id_vice {
        None => return Ok(None),
        Some(id) => id,
    };

    let repositorio_de_usuarios = RepositorioDeUsuariosSQLX::novo(db_conn);

    let usuario = match repositorio_de_usuarios
        .encontre_usuario_modelo_pelo_id(&id_vice)
        .await?
    {
        None => return Ok(None),
        Some(usuario) => usuario,
    };

    let professor = Professor::try_from(&usuario).map_err(|err| {
        ErroDeDominio::valor_invalido(
            "Não é possível associar um usuário não-professor como vice-coordenador de uma vaga.",
        )
    })?;

    Ok(Some(professor))
}
