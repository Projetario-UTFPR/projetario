use actix_web::web::{Data, Json};
use actix_web::{HttpRequest, web};
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use sqlx::PgPool;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::vagas::servicos::criar_vaga::CriarVagaParams;
use crate::infra::dtos::vagas::criar_vagas::CriarVagaDto;
//FIXME: Descomentar e ajustar quando busca em projeto estiver implementado em sqlx
//use crate::infra::fabricas::servicos::criar_vaga::obtenha_servico_criar_vaga;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::usuario_da_requisicao::UsuarioDaRequisicao;

pub struct ControllerVagas;
/* FIXME: Descomentar e ajustar quando busca em projeto estiver implementada
impl Controller for ControllerVagas {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/vagas").route("/nova", web::post().to(Self::criar)));
    }
}

impl ControllerVagas {
    pub async fn criar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        body: Json<CriarVagaDto>,
        usuario: UsuarioDaRequisicao,
    ) -> RedirectDoApp {
        let servico = obtenha_servico_criar_vaga(&db_conn);

        let professor = match usuario {
            UsuarioDaRequisicao::Professor(prof) => prof,
            _ => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Somente professores podem criar vagas.".into()],
                );
            }
        };

        // FIXME: Falta implementar busca em projeto
        let projeto = match Projeto::buscar_por_id(&db_conn, &body.id_projeto).await {
            Ok(Some(projeto)) => projeto,
            Ok(None) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Projeto não encontrado.".into()],
                );
            }
            Err(_) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap!["erro" => "Erro ao buscar projeto.".into()],
                );
            }
        };

        let vice_coordenador = if let Some(id) = &body.id_vice_coordenador {
            match Professor::buscar_por_id(&db_conn, id).await {
                Ok(Some(prof)) => Some(prof),
                Ok(None) => {
                    return Inertia::back_with_errors(
                        &req,
                        hashmap!["erro" => "Vice-coordenador não encontrado.".into()],
                    );
                }
                Err(_) => {
                    return Inertia::back_with_errors(
                        &req,
                        hashmap!["erro" => "Erro ao buscar vice-coordenador.".into()],
                    );
                }
            }
        } else {
            None
        };

        let params = CriarVagaParams {
            usuario: &professor.usuario_modelo(),
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

        if let Err(erro) = servico.executar(params).await {
            return Inertia::back_with_errors(&req, hashmap!["erro" => erro.mensagem().into()]);
        }

        Inertia::back(&req)
    }
}*/
