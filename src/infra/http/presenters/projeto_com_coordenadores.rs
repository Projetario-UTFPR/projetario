use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use serde::Serialize;

use crate::infra::http::presenters::projeto::ProjetoPresenter;
use crate::infra::http::presenters::usuario_modelo::UsuarioModeloPresenter;

#[derive(Serialize)]
pub struct ProjetoComCoordenadoresPresenter {
    projeto: ProjetoPresenter,
    coordenador: UsuarioModeloPresenter,
    #[serde(rename = "viceCoordenador")]
    vice_coordenador: Option<UsuarioModeloPresenter>,
}

impl ProjetoComCoordenadoresPresenter {
    pub fn apresente(projeto: &ProjetoComCoordenadores) -> Self {
        Self {
            coordenador: UsuarioModeloPresenter::apresente(
                &projeto.obtenha_coordenador().usuario_modelo(),
            ),
            vice_coordenador: projeto
                .obtenha_vice_coordenador()
                .map(Professor::usuario_modelo)
                .map(|usuario| UsuarioModeloPresenter::apresente(&usuario)),
            projeto: ProjetoPresenter::apresente(projeto.obtenha_projeto()),
        }
    }
}
