use std::borrow::Cow;

use dominio::comum::filtragem::DirecaoOrdenacao;
use dominio::projetos::filtragem::OrdenacaoDeProjeto;
use serde::Deserialize;
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Deserialize)]
pub struct OrdenacaoDto {
    pub ordenar_por: String,
    pub direcao: String,
}

impl OrdenacaoDto {
    pub fn obtenha_ordenacao(&self) -> OrdenacaoDeProjeto {
        let direcao = match self.direcao.as_str() {
            "asc" => DirecaoOrdenacao::Asc,
            "desc" => DirecaoOrdenacao::Desc,
            _ => panic!(
                "O programa não deveria permitir nenhuma variação inválida de direção de ordenação."
            ),
        };

        match self.ordenar_por.as_str() {
            "titulo" => OrdenacaoDeProjeto::Titulo(direcao),
            "data" => OrdenacaoDeProjeto::Data(direcao),
            _ => {
                panic!("O programa não deveria permitir nenhuma variação inválida dos ordenadores.")
            }
        }
    }
}

impl Validate for OrdenacaoDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut erros = ValidationErrors::new();

        match self.direcao.as_str() {
            "asc" => (),
            "desc" => (),
            _ => {
                let mut erro = ValidationError::new("direcao_de_ordenacao_invalida").with_message(
                    Cow::Borrowed("A direção da ordenação fornecida é inválida."),
                );

                erro.add_param(Cow::Borrowed("value"), &self.direcao);
                erro.add_param(Cow::Borrowed("valores_permitidos"), &["asc", "desc"]);
                erros.add("direcao", erro);
            }
        };

        match self.ordenar_por.as_str() {
            "titulo" => (),
            "data" => (),
            _ => {
                let mut erro = ValidationError::new("ordenador_invalido")
                    .with_message(Cow::Borrowed("O ordenador fornecido é inválido."));

                erro.add_param(Cow::Borrowed("value"), &self.ordenar_por);
                erro.add_param(Cow::Borrowed("valores_permitidos"), &["titulo", "data"]);
                erros.add("ordenar_por", erro);
            }
        };

        if erros.is_empty() {
            return Ok(());
        }

        Err(erros)
    }
}
