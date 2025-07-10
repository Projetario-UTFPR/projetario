use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

// TODO: tornar todos esses campos opcionais e utilizar o #[validate(required)]
#[derive(Deserialize, Validate, Debug)]
pub struct CriarVagaDto {
    pub id_projeto: Uuid,
    pub horas_por_semana: u8,

    #[validate(url(message = "A imagem precisa ser um URL válido."))]
    pub imagem: Option<String>,

    pub quantidade: u8,

    #[validate(url(message = "O link do edital deve ser um URL válido."))]
    pub link_edital: String,

    pub conteudo: String,

    #[validate(length(min = 5, message = "O título precisa ter, no mínimo, 5 caracteres."))]
    pub titulo: String,

    #[validate(url(message = "O link de candidatura deve ser um URL válido."))]
    pub link_candidatura: Option<String>,

    pub inscricoes_ate: NaiveDateTime,
}
