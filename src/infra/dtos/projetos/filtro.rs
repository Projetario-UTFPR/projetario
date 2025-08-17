use std::borrow::Cow;

use dominio::projetos::filtragem::FiltroDeProjeto;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize)]
pub struct FiltroDto {
    pub titulo: Option<String>,
    pub coordenacao: Option<String>,
}

impl FiltroDto {
    pub fn obtenha_filtro(&self) -> Option<FiltroDeProjeto> {
        if let Some(titulo) = self.titulo.as_deref() {
            return Some(FiltroDeProjeto::Titulo(titulo.into()));
        }

        if let Some(id_coordenador) = self.coordenacao.as_deref() {
            let id_coordenador = Uuid::try_parse(id_coordenador).expect(
                "O programa não deveria permitir que o filtro por id de coordenador recebesse uma string não-uuid."
            );

            return Some(FiltroDeProjeto::Coordenacao(id_coordenador));
        }

        None
    }
}

impl Validate for FiltroDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut erros = ValidationErrors::new();

        if self.coordenacao.is_some() && self.titulo.is_some() {
            let erro = ValidationError::new("filtro_malformado").with_message(Cow::Borrowed(
                "Não é permitido adicionar mais de 1 filtro simultâneamente.",
            ));
            erros.add("filtro", erro);
        }

        if let Some(id_coordenador) = &self.coordenacao {
            if Uuid::try_parse(id_coordenador).is_err() {
                let erro = ValidationError::new("uuid_invalido").with_message(Cow::Borrowed(
                    "Para filtrar projetos pelo coordenador, é necessário inserir um UUID válido.",
                ));
                erros.add("coordenacao", erro);
            }
        }

        Ok(())
    }
}
