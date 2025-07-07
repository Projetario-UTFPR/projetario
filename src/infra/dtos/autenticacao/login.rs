use serde::Deserialize;
use validator::Validate;

use crate::libs::validator::registro_de_aluno::validator_valide_registro_de_aluno;

#[derive(Deserialize, Validate)]
pub struct LoginDto {
    #[validate(email(message = "O campo e-mail precisa receber um e-mail válido."))]
    pub email: Option<String>,

    #[validate(custom(function = validator_valide_registro_de_aluno))]
    pub registro_aluno: Option<String>,

    #[validate(required(message = "A senha é um campo obrigatório."))]
    pub senha: Option<String>,
}
