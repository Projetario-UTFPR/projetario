use chrono::NaiveDate;
use comum::sqlx::DbDateTime;
use dominio::projetos::entidades::projeto::Projeto;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ProjetoPresenter {
    id: Uuid,
    titulo: String,
    descricao: String,
    tipo: TipoDeProjeto,
    #[serde(rename = "registradoEm")]
    registrado_em: DbDateTime,
    #[serde(rename = "iniciadoEm")]
    iniciado_em: NaiveDate,
    #[serde(rename = "atualizadoEm")]
    atualizado_em: Option<DbDateTime>,
    #[serde(rename = "canceladoEm")]
    cancelado_em: Option<DbDateTime>,
    #[serde(rename = "concluidoEm")]
    concluido_em: Option<NaiveDate>,
}

impl ProjetoPresenter {
    pub fn apresente(projeto: &Projeto) -> Self {
        Self {
            atualizado_em: projeto.obtenha_data_de_modificacao(),
            cancelado_em: projeto.obtenha_data_de_cancelamento(),
            concluido_em: projeto.obtenha_data_de_conclusao(),
            descricao: projeto.obtenha_descricao().to_owned(),
            id: projeto.obtenha_id().to_owned(),
            iniciado_em: projeto.obtenha_data_de_inicio(),
            registrado_em: projeto.obtenha_data_de_registro(),
            tipo: projeto.obtenha_tipo(),
            titulo: projeto.obtenha_titulo().to_owned(),
        }
    }
}
