use chrono::NaiveDate;
use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::vagas::entidades::vaga::Vaga;
use serde::Serialize;
use uuid::Uuid;

use crate::infra::http::presenters::usuario_modelo::UsuarioModeloPresenter;

#[derive(Serialize)]
pub struct PreviewDeVagaPresenter {
    id: Uuid,
    imagem: String,
    titulo: String,
    conteudo: String,
    #[serde(rename = "tipoDoProjeto")]
    tipo_do_projeto: TipoDeProjeto,
    #[serde(rename = "iniciadaEm")]
    iniciada_em: NaiveDate,
    coordenador: UsuarioModeloPresenter,
    #[serde(rename = "viceCoordenador")]
    vice_coordenador: Option<UsuarioModeloPresenter>,
}

impl PreviewDeVagaPresenter {
    pub fn apresente(vaga: &Vaga) -> Self {
        let coordenador =
            UsuarioModeloPresenter::apresente(&vaga.obtenha_coordenador().usuario_modelo());

        let vice_coordenador = vaga
            .obtenha_vice_coordenador()
            .map(Professor::usuario_modelo)
            .map(|usuario| UsuarioModeloPresenter::apresente(&usuario));

        Self {
            id: vaga.obtenha_id().to_owned(),
            conteudo: vaga.obtenha_conteudo().to_owned(),
            imagem: vaga.obtenha_imagem().to_owned(),
            iniciada_em: vaga.obtenha_data_de_inicio(),
            titulo: vaga.obtenha_titulo().to_owned(),
            tipo_do_projeto: vaga.obtenha_projeto().obtenha_tipo(),
            coordenador,
            vice_coordenador,
        }
    }
}
