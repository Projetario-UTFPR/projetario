use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;

#[derive(Debug, Clone)]
pub struct Vaga {
    id: Uuid,
    projeto: Uuid,
    aluno: Uuid,
    horas_por_semana: u8,
    publicada_em: NaiveDateTime,
    iniciada_em: NaiveDate,
    atualizada_em: Option<NaiveDateTime>,
    cancelada_em: Option<NaiveDateTime>,
    concluida_em: Option<NaiveDate>,
    tipo: TipoDeProjeto,
    cursos: Vec<String>,
    imagem: Option<String>,
}

impl Vaga {
    pub fn nova(
        projeto: Uuid,
        aluno: Uuid,
        horas_por_semana: u8,
        tipo: TipoDeProjeto,
        cursos: Vec<String>,
        imagem: Option<String>,
    ) -> Self {
        Self::nova_com_data_de_inicio(
            projeto,
            aluno,
            horas_por_semana,
            tipo,
            cursos,
            imagem,
            Utc::now().date_naive(),
        )
    }

    pub fn nova_com_data_de_inicio(
        projeto: Uuid,
        aluno: Uuid,
        horas_por_semana: u8,
        tipo: TipoDeProjeto,
        cursos: Vec<String>,
        imagem: Option<String>,
        iniciada_em: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            projeto,
            aluno,
            horas_por_semana,
            tipo,
            cursos,
            imagem,
            iniciada_em,
            atualizada_em: None,
            cancelada_em: None,
            concluida_em: None,
            publicada_em: Utc::now().naive_utc(),
        }
    }
}

// getters
impl Vagas {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Uuid { &self.projeto }

    pub fn obtenha_aluno(&self) -> &Uuid { &self.aluno }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana }

    pub fn obtenha_tipo(&self) -> TipoDeProjeto { self.tipo }

    pub fn obtenha_cursos(&self) -> Vec<String> { self.cursos }

    pub fn obtenha_imagem(&self) -> Option<String> { self.imagem }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_conclusao(&self) -> Option<NaiveDate> { self.concluida_em }

    pub fn obtenha_data_de_publicacao(&self) -> NaiveDateTime { self.publicada_em }

    pub fn esta_ativa(&self) -> bool { self.cancelada_em.is_none() && self.concluida_em.is_none() }
}
