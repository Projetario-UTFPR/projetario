use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;

#[derive(Debug, Clone)]
pub struct Vaga {
    id: Uuid,
    projeto: Projeto,
    coordenador: Professor,
    vice_coordenador: Option<Professor>,
    pub(crate) horas_por_semana: u8,
    pub(crate) //cursos: Vec<String>,
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
    ) -> Self {
        Self::nova_com_data_de_inicio(
            projeto,
            coordenador,
            vice_coordenador,
            horas_por_semana,
            imagem,
            quantidade,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            inscricoes_ate,
            Utc::now().date_naive(),
        )
    }

    pub fn nova_com_data_de_inicio(
        projeto: Projeto,
        coordenador: Professor,
        vice_coordenador: Option<Professor>,
        horas_por_semana: u8,
        //cursos: Vec<String>,
        imagem: Option<String>,
        quantidade: u8,
        link_edital: String,
        conteudo: String,
        titulo: String,
        link_candidatura: Option<String>,
        inscricoes_ate: NaiveDateTime,
        iniciada_em: NaiveDate,
    ) -> Self {
        if horas_por_semana == 0 || horas_por_semana > 40 {
            panic!("Horas por semana devem estar entre 1 e 40");
        }

        if quantidade == 0 {
            panic!("Quantidade deve ser pelo menos 1");
        }

        if link_edital.is_empty() {
            panic!("Link do edital não pode ser vazio");
        }

        if titulo.is_empty() {
            panic!("Título não pode ser vazio");
        } else if titulo.len() > 100 {
            panic!("Título não pode exceder 100 caracteres");
        }

        if conteudo.is_empty() {
            panic!("Conteúdo não pode ser vazio");
        }

        if inscricoes_ate < Utc::now().naive_utc() {
            panic!("Data de fechamento de inscrições não pode ser no passado");
        }

        if iniciada_em < Utc::now().date_naive() {
            panic!("Data de início não pode ser no passado");
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

    pub fn cancelar(&mut self) -> Result<(), String> {
        if self.concluida_em.is_some() {
            return Err("Não é possível cancelar uma vaga concluída".to_string());
        }
        self.cancelada_em = Some(Utc::now().naive_utc());
        self.toque();
        Ok(())
    }

    pub fn reabrir(&mut self) -> Result<(), String> {
        if self.concluida_em.is_some() {
            return Err("Não é possível reabrir uma vaga concluída".to_string());
        }
        self.cancelada_em = None;
        self.toque();
        Ok(())
    }

    pub fn permite_edicao_por(&self, professor: &Professor) -> bool {
        let professor_id = professor.obtenha_usuario().obtenha_id();
        
      
        if professor_id == self.coordenador.obtenha_usuario().obtenha_id() {
            return true;
        }
        
       
        if let Some(vice) = &self.vice_coordenador {
            if professor_id == vice.obtenha_usuario().obtenha_id() {
                return true;
            }
        }
        
        
        professor.obtenha_cargo() == &crate::dominio::identidade::enums::cargo::Cargo::Administrador
    }

    pub fn esta_ativa(&self) -> bool {
        self.cancelada_em.is_none() && self.concluida_em.is_none()
    }
}

// getters
impl Vaga {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Projeto { &self.projeto }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana }

    //pub fn obtenha_cursos(&self) -> Vec<String> { self.cursos }

    pub fn obtenha_imagem(&self) -> Option<String> { self.imagem.clone() }

    pub fn obtenha_quantidade(&self) -> u8 { self.quantidade }

    pub fn obtenha_link_edital(&self) -> String { self.link_edital.clone() }

    pub fn obtenha_link_candidatura(&self) -> Option<String> { self.link_candidatura.clone() }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_conclusao(&self) -> Option<NaiveDate> { self.concluida_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDate { self.iniciada_em }

    pub fn obtenha_data_final_inscricoes(&self) -> NaiveDateTime { self.inscricoes_ate }

    //pub fn esta_ativa(&self) -> bool { self.cancelada_em.is_none() && self.concluida_em.is_none() }

    pub fn inscricoes_ate(&self) -> NaiveDateTime { self.inscricoes_ate }

    pub fn iniciada_em(&self) -> NaiveDate { self.iniciada_em }

    pub fn atualizada_em(&self) -> Option<NaiveDateTime> { self.atualizada_em }

    pub fn cancelada_em(&self) -> Option<NaiveDateTime> { self.cancelada_em }

    pub fn concluida_em(&self) -> Option<NaiveDate> { self.concluida_em }

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

    pub fn definir_horas_por_semana(&mut self, horas: u8) -> Result<(), String> {
        if horas == 0 || horas > 40 {
            return Err("Horas por semana devem estar entre 1 e 40".to_string());
        }
        self.horas_por_semana = horas;
        self.toque();
        Ok(())
    }

    pub fn definir_imagem(&mut self, imagem: Option<String>) {
        self.imagem = imagem;
        self.toque();
    }

    pub fn definir_quantidade(&mut self, quantidade: u8) -> Result<(), String> {
        if quantidade == 0 {
            return Err("Quantidade deve ser pelo menos 1".to_string());
        }
        self.quantidade = quantidade;
        self.toque();
        Ok(())
    }

    pub fn definir_link_edital(&mut self, link_edital: String) -> Result<(), String> {
        if link_edital.is_empty() {
            return Err("Link do edital não pode ser vazio".to_string());
        }
        self.link_edital = link_edital;
        self.toque();
        Ok(())
    }

    pub fn definir_conteudo(&mut self, conteudo: String) -> Result<(), String> {
        if conteudo.is_empty() {
            return Err("Conteúdo não pode ser vazio".to_string());
        }
        self.conteudo = conteudo;
        self.toque();
        Ok(())
    }

    pub fn definir_titulo(&mut self, titulo: String) -> Result<(), String> {
        if titulo.is_empty() {
            return Err("Título não pode ser vazio".to_string());
        } else if titulo.len() > 100 {
            return Err("Título não pode exceder 100 caracteres".to_string());
        }
        self.titulo = titulo;
        self.toque();
        Ok(())
    }

    pub fn definir_link_candidatura(&mut self, link_candidatura: Option<String>) {
        self.link_candidatura = link_candidatura;
        self.toque();
    }

    pub fn definir_inscricoes_ate(&mut self, inscricoes_ate: NaiveDateTime) -> Result<(), String> {
        if inscricoes_ate < Utc::now().naive_utc() {
            return Err("Data de fechamento de inscrições não pode ser no passado".to_string());
        }
        self.inscricoes_ate = inscricoes_ate;
        self.toque();
        Ok(())
    }

    pub fn definir_iniciada_em(&mut self, iniciada_em: NaiveDate) -> Result<(), String> {
        if iniciada_em < Utc::now().date_naive() {
            return Err("Data de início não pode ser no passado".to_string());
        }
        self.iniciada_em = iniciada_em;
        self.toque();
        Ok(())
    }

    pub fn toque(&mut self) { self.atualizada_em = Some(Utc::now().naive_utc()); }

    pub fn concluir(&mut self) { self.concluida_em = Some(Utc::now().date_naive()); }

    //pub fn cancelar(&mut self) { self.cancelada_em = Some(Utc::now().naive_utc()); }
}