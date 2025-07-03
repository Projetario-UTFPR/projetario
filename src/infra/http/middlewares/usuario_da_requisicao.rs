use std::pin::Pin;
use std::rc::Rc;

use actix_session::SessionExt;
use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::web::Data;
use actix_web::{Error, HttpMessage, ResponseError};
use config::app::AppConfig;
use futures_util::future::{Ready, ready};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::infra::repositorios::sqlx::usuarios::RepositorioDeUsuariosSQLX;
use crate::utils::erros::ErroDeDominio;

/// # Middleware de Usuário da Requisição
/// Esse middleware é responsável por tentar extrair um possível usuário da requisição HTTP e,
/// se realmente houver um, autenticá-lo durante esta requisição.
///
/// # Erros
/// Ele retorna um erro na requisição se falhar ao obter uma conexão do banco de dados da conexão.
/// Em outros casos, como um ID inválido de usuário ou outros casos atípicos, o usuário simplesmente
/// não é conectado e a conexão se mantém como um usuário convidado.
pub struct MiddlewareUsuarioDaRequisicao;

pub enum UsuarioDaRequisicao {
    Convidado,
    Professor(Professor),
    Aluno(Aluno),
}

pub struct ServicoUsuarioDaRequisicao<S> {
    // service: Rc<S>
    service: Rc<S>,
}

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for MiddlewareUsuarioDaRequisicao
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ServicoUsuarioDaRequisicao<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServicoUsuarioDaRequisicao {
            service: Rc::new(service),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for ServicoUsuarioDaRequisicao<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        Box::pin(async move {
            let usuario = busque_usuario_da_sessao(&mut req).await;
            match usuario {
                Err(erro) => {
                    let http_res = erro.error_response().map_into_right_body();
                    let (http_req, _) = req.into_parts();
                    let res = ServiceResponse::new(http_req, http_res);

                    Ok(res)
                }

                Ok(usuario) => {
                    req.extensions_mut().insert(usuario);
                    let res = service.call(req).await?;
                    Ok(res.map_into_left_body())
                }
            }
        })
    }
}

async fn busque_usuario_da_sessao(
    req: &mut ServiceRequest,
) -> Result<UsuarioDaRequisicao, ErroDeDominio> {
    let user_id = match match req
        .get_session()
        .get::<String>(AppConfig::get().sessions_user_key)
        .unwrap_or(None)
    {
        None => return Ok(UsuarioDaRequisicao::Convidado),
        Some(id) => id,
    }
    .parse::<Uuid>()
    {
        Err(_) => return Ok(UsuarioDaRequisicao::Convidado),
        Ok(id) => id,
    };

    let db_conn = req.extract::<Data<PgPool>>().await.map_err(|err| {
        log::error!(
            "Failed to extract SeaService from request in WebAuthUserMiddleware: {}",
            err
        );

        ErroDeDominio::interno()
    })?;

    let usuario = RepositorioDeUsuariosSQLX::novo(&db_conn)
        .encontre_usuario_modelo_pelo_id(&user_id)
        .await;

    let usuario = match usuario {
        Err(_) => return Ok(UsuarioDaRequisicao::Convidado),
        Ok(usuario) => usuario,
    };

    let usuario = match usuario {
        None => return Ok(UsuarioDaRequisicao::Convidado),
        Some(usuario) => usuario,
    };

    if let Ok(professor) = Professor::try_from(&usuario) {
        return Ok(UsuarioDaRequisicao::Professor(professor));
    }

    if let Ok(aluno) = Aluno::try_from(&usuario) {
        return Ok(UsuarioDaRequisicao::Aluno(aluno));
    }

    log::warn!("Um usuário inválido (nem professor, nem aluno) tentou se autenticar: {usuario:#?}");

    Ok(UsuarioDaRequisicao::Convidado)
}
