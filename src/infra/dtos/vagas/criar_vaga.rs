use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::libs::UtcDateTime;

#[derive(Deserialize, Validate, Debug)]
pub struct CriarVagaDto {
    #[validate(required(
        message = "É necessário identificar o projeto ao qual esta vaga está associada."
    ))]
    pub id_projeto: Option<Uuid>,

    #[validate(required(
        message = "A quantidade de horas semanais é uma informação obrigatória."
    ))]
    pub horas_por_semana: Option<u8>,

    #[validate(
        required(message = "Imagem é um campo obrigatório."),
        url(message = "A imagem precisa ser um URL válido.")
    )]
    pub imagem: Option<String>,

    #[validate(required(message = "A quantidade de vagas disponíveis é um campo obrigatório."))]
    pub quantidade: Option<u8>,

    #[validate(
        url(message = "O link do edital deve ser um URL válido."),
        required(message = "O link do edital desta vaga é um campo obrigatório.")
    )]
    pub link_edital: Option<String>,

    #[validate(required(message = "O conteúdo/corpo da vaga é um campo obrigatório."))]
    pub conteudo: Option<String>,

    #[validate(length(min = 5, message = "O título precisa ter, no mínimo, 5 caracteres."))]
    pub titulo: Option<String>,

    #[validate(url(message = "O link de candidatura deve ser um URL válido."))]
    pub link_candidatura: Option<String>,

    #[validate(required(message = "A data limite para inscrições é um campo obrigatório."))]
    pub inscricoes_ate: Option<UtcDateTime>,
}
