use chrono::NaiveDateTime;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

// TODO: tornar todos esses campos opcionais e utilizar o #[validate(required)]
#[derive(Deserialize, Validate)]
pub struct CriarVagaDto {
    pub id_projeto: Uuid,
    pub id_coordenador: Uuid,
    pub id_vice_coordenador: Option<Uuid>,
    pub horas_por_semana: u8,
    pub imagem: Option<String>,
    pub quantidade: u8,
    pub link_edital: String,
    pub conteudo: String,
    pub titulo: String,
    pub link_candidatura: Option<String>,
    pub inscricoes_ate: NaiveDateTime,
}
