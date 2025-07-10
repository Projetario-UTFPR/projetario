use actix_session::SessionExt;
use actix_web::web::{Data, Json, Redirect};
use actix_web::{HttpRequest, Responder, web};
use config::app::AppConfig;
use inertia_rust::validators::InertiaValidateOrRedirect;
use inertia_rust::{Inertia, InertiaFacade, hashmap};
use inertia_sessions::helpers::flash_silently;
use sqlx::PgPool;

use crate::dominio::autenticacao::servicos::autenticar_usuario::{
    AutenticarUsuarioParams,
    AutenticarUsuarioResult,
    TipoDeLogin,
};
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::infra::dtos::autenticacao::LoginDto;
use crate::infra::fabricas::servicos::autenticacao::obtenha_servico_autenticar_usuario;
use crate::infra::http::controllers::{Controller, RedirectDoApp, RespostaDoApp};
use crate::infra::http::middlewares::somente_com_cargo::{
    AutorizacaoDaRota,
    MiddlewareEstaAutorizado,
};
use crate::unwrap_or_redirect;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};

pub struct ControllerAutenticacao;

impl Controller for ControllerAutenticacao {
    fn register(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(
            web::scope("/autenticacao")
                .route(
                    "/login",
                    web::post()
                        .to(Self::autenticar)
                        .wrap(MiddlewareEstaAutorizado::novo(
                            AutorizacaoDaRota::SomenteConvidado,
                        )),
                )
                .route("/login", web::get().to(Self::logar))
                .route(
                    "/logout",
                    web::post()
                        .to(Self::deslogar)
                        .wrap(MiddlewareEstaAutorizado::novo(
                            AutorizacaoDaRota::ProibirConvidado,
                        )),
                ),
        );
    }
}

impl ControllerAutenticacao {
    async fn logar(req: HttpRequest) -> impl Responder {
        Inertia::render(&req, "autenticacao/login".into()).await
    }

    async fn autenticar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        body: Json<LoginDto>,
    ) -> RedirectDoApp {
        let body = unwrap_or_redirect!(body.validate_or_back(&req));
        let service_de_autenticacao = obtenha_servico_autenticar_usuario(&db_conn);

        // Deve haver ou email ou RA, nunca os dois ou nenhum.
        if !(body.email.is_some() ^ body.registro_aluno.is_some()) {
            return Inertia::back_with_errors(
                &req,
                hashmap![
                    "error" =>  "Para logar, você deve fornecer, \
                                ou um e-mail válido, ou um registro de aluno.".into()
                ],
            );
        }

        let login = if body.email.is_some() {
            TipoDeLogin::EmailInstitucional(body.email.as_ref().unwrap())
        } else {
            TipoDeLogin::RegistroDeAluno(body.registro_aluno.as_ref().unwrap())
        };

        let resposta_autenticacao = service_de_autenticacao
            .executar(AutenticarUsuarioParams {
                login,
                // é seguro utilizar `unwrap` nesse caso porque o validator garante que a senha
                // é `Some`; nunca `None`.
                senha: &body.senha.unwrap(),
            })
            .await;

        let autenticacao = match resposta_autenticacao {
            Ok(autenticacao) => autenticacao,
            Err(erro) => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap![
                        "error" => erro.mensagem().into()
                    ],
                );
            }
        };

        let usuario = match autenticacao {
            AutenticarUsuarioResult::Autenticado(usuario) => usuario,
            AutenticarUsuarioResult::NaoAutenticado => {
                return Inertia::back_with_errors(
                    &req,
                    hashmap![ "error" => "Credenciais inválidas.".into() ],
                );
            }
        };

        let nome_usuario = usuario.nome.clone();

        unwrap_or_redirect!(colocar_usuario_nas_sessoes(&req, usuario).map_err(|erro| {
            Inertia::back_with_errors(&req, hashmap!["error" => erro.mensagem().into()])
        }));

        flash_silently(
            &req,
            "sucessoLogin",
            format!("Você foi autenticado com sucesso como {nome_usuario}!"),
        );

        Redirect::to("/").see_other()
    }

    async fn deslogar(req: HttpRequest) -> RedirectDoApp {
        remover_usuario_das_sessoes(&req);
        Redirect::to("/").see_other()
    }
}

fn colocar_usuario_nas_sessoes(req: &HttpRequest, usuario: UsuarioModelo) -> ResultadoDominio<()> {
    req
        .get_session()
        .insert(AppConfig::get().sessions_user_key, usuario)
        .map_err(|erro| {
            log::error!("Um erro inesperado ocorreu ao guardar o json do usuário autenticado nas sessões: {erro}");
            ErroDeDominio::interno()
        })
}

fn remover_usuario_das_sessoes(req: &HttpRequest) {
    req.get_session().remove(AppConfig::get().sessions_user_key);
}
