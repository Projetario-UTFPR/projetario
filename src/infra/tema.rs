use std::convert::Infallible;
use std::pin::Pin;

use actix_web::cookie::{Cookie, SameSite};
use actix_web::{FromRequest, HttpRequest, HttpResponse};
use comum::erros::{ErroDeDominio, ResultadoDominio};

mod enums;
pub use enums::*;

const COOKIE_DO_TEMA: &str = "projetario_favourite_theme";
const COOKIE_DO_TEMA_PREFERIDO: &str = "projetario_user_sys_theme";

#[derive(Clone)]
pub struct GerenteDeTema {
    production: bool,
}

impl GerenteDeTema {
    pub fn new(production: bool) -> Self { Self { production } }

    /// Extrai uma instância de `TemaDaAplicacao` dos cookies, se existir.
    pub fn extraia_tema(&self, req: &HttpRequest) -> Option<TemaDaAplicacao> {
        req.cookie(COOKIE_DO_TEMA)
            .map(|cookie| TemaDaAplicacao::try_from(cookie.value()))?
            .map(Option::Some)
            .unwrap_or_default()
    }

    pub fn extraia_tema_do_sistema(&self, req: &HttpRequest) -> Option<TemaDaAplicacaoEstrito> {
        req.cookie(COOKIE_DO_TEMA_PREFERIDO)
            .map(|cookie| TemaDaAplicacaoEstrito::try_from(cookie.value()))?
            .map(Option::Some)
            .unwrap_or_default()
    }

    /// Exatamente como o `GerenteDeTema::persista_tema`, porém
    /// não retorna erro algum quando falha (apenas o ignora).
    pub fn persista_tema_silenciosamente(&self, res: &mut HttpResponse, tema: TemaDaAplicacao) {
        let _ = self.persista_tema(res, tema);
    }

    /// Exatamente como o `GerenteDeTema::persista_tema_do_sistema`, porém
    /// não retorna erro algum quando falha (apenas o ignora).
    pub fn persista_tema_do_sistema_silenciosamente(
        &self,
        res: &mut HttpResponse,
        tema: TemaDaAplicacaoEstrito,
    ) {
        let _ = self.persista_tema_do_sistema(res, tema);
    }

    /// Insere o tema em um cookie na resposta HTTP. Retorna um erro interno se falhar
    /// em persistir o cookie e imprime o erro original nos logs.
    pub fn persista_tema(
        &self,
        res: &mut HttpResponse,
        tema: TemaDaAplicacao,
    ) -> ResultadoDominio<()> {
        self.salve_tema_no_cookie(COOKIE_DO_TEMA, res, tema)
    }

    /// Insere o tema do sistema do usuário em um cookie na resposta HTTP. Retorna um erro interno
    /// se falhar em persistir o cookie e imprime o erro original nos logs.
    pub fn persista_tema_do_sistema(
        &self,
        res: &mut HttpResponse,
        tema: TemaDaAplicacaoEstrito,
    ) -> ResultadoDominio<()> {
        self.salve_tema_no_cookie(COOKIE_DO_TEMA_PREFERIDO, res, tema.into())
    }

    fn make_actix_cookie<'a>(&self, key: &'a str, tema: TemaDaAplicacao) -> Cookie<'a> {
        let mut cookie = Cookie::new(key, tema.to_string());
        cookie.set_same_site(SameSite::Lax);
        cookie.set_http_only(true);
        cookie.set_path("/");
        cookie.set_secure(self.production);
        cookie
    }

    pub fn empreste_da_requisicao(req: &HttpRequest) -> &Self {
        req.app_data::<GerenteDeTema>()
            .expect("O `GerenteDeTema` não foi inserido como um `AppData` no servidor HTTP.")
    }

    fn salve_tema_no_cookie(
        &self,
        key: &str,
        res: &mut HttpResponse,
        tema: TemaDaAplicacao,
    ) -> ResultadoDominio<()> {
        if let Err(err) = res.add_cookie(&self.make_actix_cookie(key, tema)) {
            log::warn!("Não foi possível persistir o cookie de tema para um usuário: {err}");
            return Err(ErroDeDominio::interno().com_mensagem(err.to_string()));
        }

        Ok(())
    }
}

impl FromRequest for GerenteDeTema {
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let gerente = match req.app_data::<GerenteDeTema>() {
            None => {
                panic!("O `GerenteDeTema` não foi inserido como um `AppData` no servidor HTTP.")
            }
            Some(gerente) => gerente,
        }
        .clone();

        Box::pin(async move { Ok(gerente) })
    }
}
