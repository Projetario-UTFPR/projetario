use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;

#[derive(Serialize)]
pub struct UsuarioModeloPresenter {
    pub id: Uuid,
    pub nome: String,
    pub email: String,

    #[serde(rename = "urlCurriculoLattes")]
    pub url_curriculo_lattes: Option<String>,
    pub cargo: Cargo,

    #[serde(rename = "registradoEm")]
    pub registrado_em: NaiveDateTime,

    #[serde(rename = "atualizadoEm")]
    pub atualizado_em: Option<NaiveDateTime>,

    #[serde(rename = "registroAluno")]
    pub registro_aluno: Option<String>,
    pub periodo: Option<i16>,
}

impl UsuarioModeloPresenter {
    pub fn apresente(usuario: &UsuarioModelo) -> Self {
        Self {
            atualizado_em: usuario.atualizado_em,
            cargo: usuario.cargo.clone(),
            email: usuario.email.clone(),
            id: usuario.id,
            nome: usuario.nome.to_owned(),
            periodo: usuario.periodo,
            registrado_em: usuario.registrado_em,
            registro_aluno: usuario.registro_aluno.clone(),
            url_curriculo_lattes: usuario.url_curriculo_lattes.clone(),
        }
    }
}
