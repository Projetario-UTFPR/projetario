use std::borrow::Cow;

use inertia_rust::hashmap;
use validator::ValidationError;

use crate::dominio::identidade::politicas::registro_de_aluno::valide_registro_de_aluno;
use crate::libs::validator::CODIGO_DE_ERRO_VALIDACAO_DE_DOMINIO;

pub fn validator_valide_registro_de_aluno(ra: &str) -> Result<(), ValidationError> {
    let eh_valido = valide_registro_de_aluno(ra);

    if eh_valido {
        return Ok(());
    }

    Err(ValidationError {
        code: Cow::Borrowed(CODIGO_DE_ERRO_VALIDACAO_DE_DOMINIO),
        message: Some(Cow::Borrowed(
            "O registro de aluno ^recisa ser um valor^numérico prefixado com 'a'.",
        )),
        params: hashmap![],
    })
}
