use std::pin::Pin;
use std::rc::Rc;

use actix_web::body::EitherBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready};
use actix_web::{Error, HttpMessage, ResponseError};
use futures_util::future::{Ready, ready};

use crate::dominio::autenticacao::politicas::autorizacao::PoliticasDeAutorizacao;
use crate::dominio::identidade::entidades::usuario;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::infra::http::middlewares::usuario_da_requisicao::{self, UsuarioDaRequisicao};
use crate::utils::erros::ErroDeDominio;

#[derive(Clone, PartialEq)]
pub enum AutorizacaoDaRota {
    SomenteConvidado,
    ProibirConvidado,
    UsuarioComCargo(Cargo),
    QualquerUm,
}

pub struct MiddlewareEstaAutorizado {
    autorizacao: AutorizacaoDaRota,
}

impl MiddlewareEstaAutorizado {
    pub fn novo(autorizacao: AutorizacaoDaRota) -> Self { Self { autorizacao } }
}

pub struct ServicoEstaAutorizado<S> {
    service: S,
    autorizacao: AutorizacaoDaRota,
}

// S: 'static if working with async
impl<S, B> Transform<S, ServiceRequest> for MiddlewareEstaAutorizado
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ServicoEstaAutorizado<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ServicoEstaAutorizado {
            service,
            autorizacao: self.autorizacao.clone(),
        }))
    }
}

impl<S, B> Service<ServiceRequest> for ServicoEstaAutorizado<S>
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
        let extensions = req.extensions();
        let usuario_da_req = extensions.get::<UsuarioDaRequisicao>().expect(
            "O `MiddlewareSmoenteComCargo` exige que o middleware \
            `MiddlewareUsuarioDaRequisicao` tenha sido corretamente configurado para funcionar.",
        );

        let usuario_esta_autorizado = match &self.autorizacao {
            AutorizacaoDaRota::QualquerUm => true,
            AutorizacaoDaRota::ProibirConvidado => {
                usuario_da_req.ne(&UsuarioDaRequisicao::Convidado)
            }
            AutorizacaoDaRota::SomenteConvidado => {
                usuario_da_req.eq(&UsuarioDaRequisicao::Convidado)
            }
            AutorizacaoDaRota::UsuarioComCargo(cargo) => match &usuario_da_req {
                UsuarioDaRequisicao::Convidado => false,
                UsuarioDaRequisicao::Aluno(_) => {
                    PoliticasDeAutorizacao::hierarquia_do_cargo_permite(&Cargo::Aluno, cargo)
                }
                UsuarioDaRequisicao::Professor(professor) => {
                    PoliticasDeAutorizacao::hierarquia_do_cargo_permite(
                        professor.obtenha_cargo(),
                        cargo,
                    )
                }
            },
        };

        drop(extensions);

        if !usuario_esta_autorizado {
            let resposta_erro_http =
                ErroDeDominio::nao_autorizado("Você não está autorizado a acessar esta rota.")
                    .error_response()
                    .map_into_right_body();

            let (requisicao, _) = req.into_parts();

            return Box::pin(
                async move { Ok(ServiceResponse::new(requisicao, resposta_erro_http)) },
            );
        }

        let futuro = self.service.call(req);
        Box::pin(async move { Ok(futuro.await?.map_into_left_body()) })
    }
}
