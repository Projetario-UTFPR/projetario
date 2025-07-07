use chrono::NaiveDate;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CriarProjetoDto {
    #[validate(
        required(message = "Título é um campo obrigatório."),
        length(
            max = 200,
            message = "O título do projeto deve ter no máximo 200 caracteres."
        )
    )]
    pub titulo: Option<String>,

    #[validate(
        required(message = "Descrição é um campo obrigatório."),
        length(
            min = 50,
            message = "A descrição do seu projeto deveria ter, no mínimo, 50 caracteres."
        )
    )]
    pub descricao: Option<String>,

    pub data_de_inicio: Option<NaiveDate>,
}
