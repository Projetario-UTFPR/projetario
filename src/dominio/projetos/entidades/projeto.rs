use chrono::{NaiveDate, NaiveDateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;

#[derive(Debug, Clone, FromRow)]
pub struct Projeto {
    id: Uuid,
    titulo: String,
    descricao: String,
    tipo: TipoDeProjeto,
    registrado_em: NaiveDateTime,
    iniciado_em: NaiveDate,
    atualizado_em: Option<NaiveDateTime>,
    cancelado_em: Option<NaiveDateTime>,
    concluido_em: Option<NaiveDate>,
}

impl Projeto {
    pub fn novo(titulo: String, descricao: String, tipo: TipoDeProjeto) -> Self {
        Self::novo_com_data_de_inicio(titulo, descricao, tipo, Utc::now().date_naive())
    }

    pub fn novo_com_data_de_inicio(
        titulo: String,
        descricao: String,
        tipo: TipoDeProjeto,
        iniciado_em: NaiveDate,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            titulo,
            descricao,
            tipo,
            atualizado_em: None,
            cancelado_em: None,
            concluido_em: None,
            iniciado_em,
            registrado_em: Utc::now().naive_utc(),
        }
    }
}

// getters
impl Projeto {
    pub fn obtenha_id(&self) -> &Uuid { &self.id }

    pub fn obtenha_titulo(&self) -> &str { &self.titulo }

    pub fn obtenha_descricao(&self) -> &str { &self.descricao }

    pub fn obtenha_tipo(&self) -> TipoDeProjeto { self.tipo }

    pub fn obtenha_data_de_registro(&self) -> NaiveDateTime { self.registrado_em }

    pub fn obtenha_data_de_inicio(&self) -> NaiveDate { self.iniciado_em }

    pub fn obtenha_data_de_cancelamento(&self) -> Option<NaiveDateTime> { self.cancelado_em }

    pub fn obtenha_data_de_conclusao(&self) -> Option<NaiveDate> { self.concluido_em }

    pub fn obtenha_data_de_modificacao(&self) -> Option<NaiveDateTime> { self.atualizado_em }

    pub fn esta_ativo(&self) -> bool { self.cancelado_em.is_none() && self.concluido_em.is_none() }
}

// setters
impl Projeto {
    pub fn coloque_titulo(&mut self, titulo: String) {
        if self.titulo == titulo {
            return;
        }

        self.titulo = titulo;
        self.toque();
    }

    pub fn coloque_descricao(&mut self, descricao: String) {
        if self.descricao == descricao {
            return;
        }

        self.descricao = descricao;
        self.toque();
    }

    pub fn toque(&mut self) { self.atualizado_em = Some(Utc::now().naive_utc()); }

    pub fn concluir(&mut self) { self.concluido_em = Some(Utc::now().date_naive()); }

    pub fn cancelar(&mut self) { self.cancelado_em = Some(Utc::now().naive_utc()); }
}
