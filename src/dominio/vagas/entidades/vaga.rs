use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::dominio::identidade::entidades::aluno::Aluno;
use crate::dominio::projetos::entidades::projeto::Projeto;

#[derive(Debug, Clone)]
pub struct Vaga {
    id: Uuid,
    projeto: Projeto,
    horas_por_semana: u8,
    //cursos: Vec<String>,
    imagem: Option<String>,
    quantidade: u8,
    link_edital: String,
    link_candidatura: Option<String>,
    titulo: String,
    conteudo: String,
    atualizada_em: Option<NaiveDateTime>,
    cancelada_em: Option<NaiveDateTime>,
    concluida_em: Option<NaiveDate>,
    iniciada_em: NaiveDate,
}

impl Vaga {
    pub fn nova() -> Self {
        Self::nova_com_data_de_inicio(
            projeto,
            horas_por_semana,
            imagem,
            quantidade,
            link_candidatura,
            conteudo,
            titulo,
            link_candidatura,
            Utc::now().date_naive(),
        )
    }

    pub fn nova_com_data_de_inicio(
        projeto: Projeto,
        horas_por_semana: u8,
        //cursos: Vec<String>,
        imagem: Option<String>,
        quantidade: u8,
        link_edital: String,
        conteudo: String,
        titulo: String,
        link_candidatura: String,
        iniciada_em: NaiveDate,
    ) -> Self {
        Self::nova_com_data_de_inicio(
            projeto,
            horas_por_semana,
            //cursos,
            imagem,
            quantidade,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            Utc::now().date_naive(),
        )
    }
}

// getters
impl Vagas {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Uuid { &self.projeto }

    pub fn obtenha_aluno(&self) -> &Uuid { &self.aluno }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana }

    pub fn obtenha_tipo(&self) -> TipoDeProjeto { self.tipo }

    //pub fn obtenha_cursos(&self) -> Vec<String> { self.cursos }

    pub fn obtenha_imagem(&self) -> Option<String> { self.imagem }

    pub fn obtenha_quantidade(&self) -> u8 { self.quantidade }

    pub fn obtenha_link_edital(&self) -> String { self.link_edital }

    pub fn obtenha_link_candidatura(&self) -> String { self.link_candidatura }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_conclusao(&self) -> Option<NaiveDate> { self.concluida_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDateTime { self.iniciada_em }

    pub fn esta_ativa(&self) -> bool { self.cancelada_em.is_none() && self.concluida_em.is_none() }
}

// setters
impl Vaga {
    pub fn coloque_conteudo(&mut self, conteudo: String) {
        if self.conteudo == conteudo {
            return;
        }

        self.conteudo = conteudo;
        self.toque();
    }

    pub fn coloque_imagem(&mut self, imagem: String) {
        if self.imagem == imagem {
            return;
        }

        self.imagem = imagem;
        self.toque();
    }

    pub fn toque(&mut self) { self.atualizada_em = Some(Utc::now().naive_utc()); }

    pub fn concluir(&mut self) { self.concluida_em = Some(Utc::now().date_naive()); }

    pub fn cancelar(&mut self) { self.cancelada_em = Some(Utc::now().naive_utc()); }
}
