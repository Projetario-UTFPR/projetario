use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::utils::erros::ResultadoDominio;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

#[derive(Debug, Clone, FromRow)]
pub struct Vaga {
    id: Uuid,

    #[sqlx(flatten)]
    projeto: Projeto,

    #[sqlx(flatten)]
    coordenador: Professor,

    #[sqlx(flatten)]
    vice_coordenador: Option<Professor>,

    horas_por_semana: i32,
    //cursos: Vec<String>,
    imagem: Option<String>,
    quantidade: i32,
    link_edital: String,
    link_candidatura: Option<String>,
    titulo: String,
    conteudo: String,
    iniciada_em: NaiveDate,
    inscricoes_ate: NaiveDateTime,
    cancelada_em: Option<NaiveDateTime>,
    atualizada_em: Option<NaiveDateTime>,
}
#[allow(clippy::too_many_arguments)]
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

        Self::valide_horas_por_semana(horas_por_semana)?;
        Self::valide_quantidade_de_vagas(quantidade)?;

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

        Self::valide_data_de_encerramento_das_inscricoes(&inscricoes_ate)?;

        Ok(Self {
            id: Uuid::new_v4(),
            projeto,
            coordenador,
            vice_coordenador,
            horas_por_semana: horas_por_semana as i32,
            //cursos,
            imagem,
            quantidade: quantidade as i32,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            atualizada_em: None,
            cancelada_em: None,
            inscricoes_ate,
            iniciada_em,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn criar_de_existente(
        id: Uuid,
        projeto: Projeto,
        coordenador: Professor,
        vice_coordenador: Option<Professor>,
        horas_por_semana: i32,
        imagem: Option<String>,
        quantidade: i32,
        link_edital: String,
        link_candidatura: Option<String>,
        titulo: String,
        conteudo: String,
        iniciada_em: NaiveDate,
        inscricoes_ate: NaiveDateTime,
        cancelada_em: Option<NaiveDateTime>,
        atualizada_em: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            atualizada_em,
            cancelada_em,
            conteudo,
            coordenador,
            horas_por_semana,
            id,
            imagem,
            iniciada_em,
            inscricoes_ate,
            link_candidatura,
            link_edital,
            projeto,
            quantidade,
            titulo,
            vice_coordenador,
        }
    }
}

// getters
impl Vaga {
    pub fn obtenha_titulo(&self) -> &str { &self.titulo }

    pub fn obtenha_conteudo(&self) -> &str { &self.conteudo }

    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Projeto { &self.projeto }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana as u8 }

    //pub fn obtenha_cursos(&self) -> Vec<String> { self.cursos }

    pub fn obtenha_imagem(&self) -> Option<String> { self.imagem.clone() }

    pub fn obtenha_quantidade(&self) -> u8 { self.quantidade as u8 }

    pub fn obtenha_link_edital(&self) -> &String { &self.link_edital }

    pub fn obtenha_link_candidatura(&self) -> Option<String> { self.link_candidatura.clone() }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDate { self.iniciada_em }

    pub fn obtenha_data_final_inscricoes(&self) -> NaiveDateTime { self.inscricoes_ate }

    pub fn foi_concluida(&self) -> bool {
        self.cancelada_em.is_none() && self.inscricoes_ate < Utc::now().naive_utc()
    }

    pub fn esta_ativa(&self) -> bool { self.cancelada_em.is_none() && !self.foi_concluida() }

    pub fn obtenha_coordenador(&self) -> &Professor { &self.coordenador }

    pub fn obtenha_vice_coordenador(&self) -> Option<&Professor> { self.vice_coordenador.as_ref() }
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
        if self.imagem.as_ref() == imagem.as_ref() {
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
        if self.link_candidatura.as_ref() == link_candidatura.as_ref() {
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

    pub fn coloque_horas_por_semana(&mut self, horas: u8) -> ResultadoDominio<()> {
        if self.horas_por_semana == horas as i32 {
            return Ok(());
        }

        Self::valide_horas_por_semana(horas)?;

        self.horas_por_semana = horas as i32;
        self.toque();

        Ok(())
    }

    pub fn coloque_quantidade_de_vagas(&mut self, qtd: u8) -> ResultadoDominio<()> {
        if self.quantidade == qtd as i32 {
            return Ok(());
        }

        Self::valide_quantidade_de_vagas(qtd);
        self.quantidade = qtd as i32;
        self.toque();

        Ok(())
    }

    pub fn atualize_data_de_encerramento_das_inscricoes(
        &mut self,
        data: NaiveDateTime,
    ) -> ResultadoDominio<()> {
        if self.inscricoes_ate == data {
            return Ok(());
        }

        Self::valide_data_de_encerramento_das_inscricoes(&data)?;
        self.inscricoes_ate = data;
        self.toque();
        Ok(())
    }

    pub fn coloque_vice_coordenador(&mut self, vice: Professor) {
        self.vice_coordenador = Some(vice);
    }

    pub fn toque(&mut self) { self.atualizada_em = Some(Utc::now().naive_utc()); }

    pub fn cancelar(&mut self) { self.cancelada_em = Some(Utc::now().naive_utc()); }
}

impl Vaga {
    pub fn valide_horas_por_semana(horas: u8) -> ResultadoDominio<()> {
        if horas == 0 || horas > 40 {
            return Err(ErroDeDominio::valor_invalido(
                "Horas por semana devem estar entre 1 e 40.",
            ));
        }

        Ok(())
    }

    pub fn valide_quantidade_de_vagas(qtd: u8) -> ResultadoDominio<()> {
        if qtd == 0 {
            return Err(ErroDeDominio::valor_invalido(
                "Quantidade deve ser pelo menos 1.",
            ));
        }

        Ok(())
    }

    pub fn valide_data_de_encerramento_das_inscricoes(
        data: &NaiveDateTime,
    ) -> ResultadoDominio<()> {
        if Utc::now().naive_utc().gt(data) {
            return Err(ErroDeDominio::valor_invalido(
                "Data de fechamento de inscrições não pode ser no passado.",
            ));
        }

        Ok(())
    }
}
