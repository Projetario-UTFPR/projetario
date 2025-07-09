use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[derive(Debug, Clone)]
pub struct Vaga {
    id: Uuid,
    projeto: Projeto,
    coordenador: Professor,
    vice_coordenador: Option<Professor>,
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
    inscricoes_ate: NaiveDateTime,
    iniciada_em: NaiveDate,
}

impl Vaga {
    pub fn nova(
        projeto: Projeto,
        coordenador: Professor,
        vice_coordenador: Option<Professor>,
        horas_por_semana: u8,
        imagem: Option<String>,
        quantidade: u8,
        link_edital: String,
        conteudo: String,
        titulo: String,
        link_candidatura: Option<String>,
        inscricoes_ate: NaiveDateTime,
    ) -> Result<Self, ErroDeDominio> {

        let iniciada_em = Utc::now().date_naive();

        if horas_por_semana == 0 || horas_por_semana > 40 {
            return Err(ErroDeDominio::valor_invalido(
                "Horas por semana devem estar entre 1 e 40.",
            ));
        }

        if quantidade == 0 {
            return Err(ErroDeDominio::valor_invalido(
                "Quantidade deve ser pelo menos 1.",
            ));
        }

        if link_edital.is_empty() {
            return Err(ErroDeDominio::valor_invalido(
                "Link do edital não pode ser vazio.",
            ));
        }

        if titulo.is_empty() {
            return Err(ErroDeDominio::valor_invalido("Título não pode ser vazio."));
        } else if titulo.len() > 100 {
            return Err(ErroDeDominio::valor_invalido(
                "Título não pode exceder 100 caracteres.",
            ));
        }

        if conteudo.is_empty() {
            return Err(ErroDeDominio::valor_invalido(
                "Conteúdo não pode ser vazio.",
            ));
        }

        if inscricoes_ate < Utc::now().naive_utc() {
            return Err(ErroDeDominio::valor_invalido(
                "Data de fechamento de inscrições não pode ser no passado.",
            ));
        }
        Self {
            id: Uuid::new_v4(),
            projeto,
            coordenador,
            vice_coordenador,
            horas_por_semana,
            //cursos,
            imagem,
            quantidade,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            atualizada_em: None,
            cancelada_em: None,
            concluida_em: None,
            inscricoes_ate,
            iniciada_em,
        }
    }
}

// getters
impl Vaga {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Projeto { &self.projeto }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana }

    //pub fn obtenha_cursos(&self) -> Vec<String> { self.cursos }

    pub fn obtenha_imagem(&self) -> Option<String> { self.imagem }

    pub fn obtenha_quantidade(&self) -> u8 { self.quantidade }

    pub fn obtenha_link_edital(&self) -> String { self.link_edital }

    pub fn obtenha_link_candidatura(&self) -> Option<String> { self.link_candidatura }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_conclusao(&self) -> Option<NaiveDate> { self.concluida_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDate { self.iniciada_em }

    pub fn obtenha_data_final_inscricoes(&self) -> NaiveDateTime { self.inscricoes_ate }

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

    pub fn coloque_imagem(&mut self, imagem: Option<String>) {
        if self.imagem.as_ref() == Some(&imagem) {
            return;
        }

        self.imagem = imagem;
        self.toque();
    }

    pub fn coloque_link_edital(&mut self, link_edital: String) {
        if self.link_edital == link_edital {
            return;
        }

        self.link_edital = link_edital;
        self.toque();
    }

    pub fn coloque_link_candidatura(&mut self, link_candidatura: Option<String>) {
        if self.link_candidatura.as_ref() == Some(&link_candidatura) {
            return;
        }

        self.link_candidatura = link_candidatura;
        self.toque();
    }

    pub fn coloque_titulo(&mut self, titulo: String) {
        if self.titulo == titulo {
            return;
        }

        self.titulo = titulo;
        self.toque();
    }

    pub fn toque(&mut self) { self.atualizada_em = Some(Utc::now().naive_utc()); }

    pub fn concluir(&mut self) { self.concluida_em = Some(Utc::now().date_naive()); }

    pub fn cancelar(&mut self) { self.cancelada_em = Some(Utc::now().naive_utc()); }
}
