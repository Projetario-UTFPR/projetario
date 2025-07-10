use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CriarVagaDto {
    pub id_projeto: String,
    pub id_coordenador: String,
    pub id_vice_coordenador: Option<String>,
    pub horas_por_semana: u8,
    pub imagem: Option<String>,
    pub quantidade: u8,
    pub link_edital: String,
    pub conteudo: String,
    pub titulo: String,
    pub link_candidatura: Option<String>,
    pub inscricoes_ate: NaiveDateTime,
}
