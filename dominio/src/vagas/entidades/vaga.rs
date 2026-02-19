use chrono::{NaiveDate, Utc};
use comum::erros::ResultadoDominio;
use comum::erros::erro_de_dominio::ErroDeDominio;
use comum::sqlx::{DbDateTime, db_date_time_now};
use uuid::Uuid;

use crate::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use crate::identidade::entidades::professor::Professor;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::entidades::projeto::Projeto;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(dev_utils, derive(derive_builder::Builder))]
#[cfg_attr(dev_utils, builder(setter(into)))]
pub struct Vaga {
    id: Uuid,
    projeto_e_coordenadores: ProjetoComCoordenadores,
    horas_por_semana: u8,
    imagem: String,
    quantidade: u8,
    link_edital: String,
    link_candidatura: Option<String>,
    /// Se um título não for fornecido, será utilizad o título do projeto ao qual
    /// esta vaga está associada.
    titulo: Option<String>,
    conteudo: String,
    iniciada_em: NaiveDate,
    inscricoes_ate: DbDateTime,
    cancelada_em: Option<DbDateTime>,
    atualizada_em: Option<DbDateTime>,
}

#[allow(clippy::too_many_arguments)]
impl Vaga {
    pub fn nova(
        projeto_e_coordenadores: ProjetoComCoordenadores,
        horas_por_semana: u8,
        imagem: String,
        quantidade_de_vagas: u8,
        link_edital: String,
        conteudo: String,
        titulo_customizado: Option<String>,
        link_candidatura: Option<String>,
        inscricoes_ate: DbDateTime,
    ) -> Result<Self, ErroDeDominio> {
        Self::valide_horas_por_semana(horas_por_semana)?;
        Self::valide_quantidade_de_vagas(quantidade_de_vagas)?;

        if link_edital.is_empty() {
            return Err(ErroDeDominio::valor_invalido(
                "Link do edital não pode ser vazio.",
            ));
        }

        if let Some(titulo) = &titulo_customizado {
            if titulo.is_empty() {
                return Err(ErroDeDominio::valor_invalido("Título não pode ser vazio."));
            } else if titulo.len() > 100 {
                return Err(ErroDeDominio::valor_invalido(
                    "Título não pode exceder 100 caracteres.",
                ));
            }
        }

        if conteudo.is_empty() {
            return Err(ErroDeDominio::valor_invalido(
                "Conteúdo não pode ser vazio.",
            ));
        }

        Self::valide_data_de_encerramento_das_inscricoes(&inscricoes_ate)?;

        Ok(Self {
            id: Uuid::new_v4(),
            projeto_e_coordenadores,
            horas_por_semana,
            imagem,
            quantidade: quantidade_de_vagas,
            link_edital,
            conteudo,
            titulo: titulo_customizado,
            link_candidatura,
            atualizada_em: None,
            cancelada_em: None,
            inscricoes_ate,
            iniciada_em: Utc::now().date_naive(),
        })
    }

    pub(crate) fn nova_de_dados_brutos(
        id: Uuid,
        projeto_e_coordenadores: ProjetoComCoordenadores,
        horas_por_semana: u8,
        imagem: String,
        quantidade: u8,
        link_edital: String,
        link_candidatura: Option<String>,
        titulo: Option<String>,
        conteudo: String,
        iniciada_em: NaiveDate,
        inscricoes_ate: DbDateTime,
        cancelada_em: Option<DbDateTime>,
        atualizada_em: Option<DbDateTime>,
    ) -> Self {
        Self {
            atualizada_em,
            cancelada_em,
            conteudo,
            projeto_e_coordenadores,
            horas_por_semana,
            id,
            imagem,
            iniciada_em,
            inscricoes_ate,
            link_candidatura,
            link_edital,
            quantidade,
            titulo,
        }
    }
}

// getters
impl Vaga {
    pub fn obtenha_titulo(&self) -> &str {
        self.titulo
            .as_deref()
            .unwrap_or_else(|| self.obtenha_projeto().obtenha_titulo())
    }

    pub fn obtenha_conteudo(&self) -> &str { &self.conteudo }

    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_projeto(&self) -> &Projeto { self.projeto_e_coordenadores.obtenha_projeto() }

    pub fn obtenha_horas_por_semana(&self) -> u8 { self.horas_por_semana }

    pub fn obtenha_imagem(&self) -> &str { &self.imagem }

    pub fn obtenha_quantidade(&self) -> u8 { self.quantidade }

    pub fn obtenha_link_edital(&self) -> &String { &self.link_edital }

    pub fn obtenha_link_candidatura(&self) -> Option<String> { self.link_candidatura.clone() }

    pub fn obtenha_data_de_modificacao(&self) -> Option<DbDateTime> { self.atualizada_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<DbDateTime> { self.cancelada_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDate { self.iniciada_em }

    pub fn obtenha_data_final_inscricoes(&self) -> DbDateTime { self.inscricoes_ate }

    pub fn foi_concluida(&self) -> bool {
        self.cancelada_em.is_none() && self.inscricoes_ate < db_date_time_now()
    }

    pub fn esta_ativa(&self) -> bool { self.cancelada_em.is_none() && !self.foi_concluida() }

    pub fn foi_cancelada(&self) -> bool { self.cancelada_em.is_some() }
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
        if self.titulo.as_ref().is_some_and(|t| t.eq(&titulo)) {
            return;
        }

        self.titulo = Some(titulo);
        self.toque();
    }

    pub fn remova_titulo_customizado(&mut self) {
        if self.titulo.is_none() {
            return;
        }

        self.titulo = None;
        self.toque();
    }

    pub fn coloque_horas_por_semana(&mut self, horas: u8) -> ResultadoDominio<()> {
        if self.horas_por_semana == horas {
            return Ok(());
        }

        Self::valide_horas_por_semana(horas)?;

        self.horas_por_semana = horas;
        self.toque();

        Ok(())
    }

    pub fn coloque_quantidade_de_vagas(&mut self, qtd: u8) -> ResultadoDominio<()> {
        if self.quantidade == qtd {
            return Ok(());
        }

        Self::valide_quantidade_de_vagas(qtd)?;
        self.quantidade = qtd;
        self.toque();

        Ok(())
    }

    pub fn atualize_data_de_encerramento_das_inscricoes(
        &mut self,
        data: DbDateTime,
    ) -> ResultadoDominio<()> {
        if self.inscricoes_ate == data {
            return Ok(());
        }

        Self::valide_data_de_encerramento_das_inscricoes(&data)?;
        self.inscricoes_ate = data;
        self.toque();
        Ok(())
    }

    pub fn toque(&mut self) { self.atualizada_em = Some(db_date_time_now()); }

    pub fn cancelar(&mut self) { self.cancelada_em = Some(db_date_time_now()); }
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

    pub fn valide_data_de_encerramento_das_inscricoes(data: &DbDateTime) -> ResultadoDominio<()> {
        if db_date_time_now().gt(data) {
            return Err(ErroDeDominio::valor_invalido(
                "Data de fechamento de inscrições não pode ser no passado.",
            ));
        }

        Ok(())
    }
}

impl AgregadoComCoordenador for Vaga {
    fn obtenha_coordenador(&self) -> &Professor {
        self.projeto_e_coordenadores.obtenha_coordenador()
    }

    fn obtenha_vice_coordenador(&self) -> Option<&Professor> {
        self.projeto_e_coordenadores.obtenha_vice_coordenador()
    }
}
