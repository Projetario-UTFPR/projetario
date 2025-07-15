use chrono::NaiveDateTime;
use fake::faker::chrono::pt_br::DateTime;
use fake::faker::internet::pt_br::{FreeEmail, Password};
use fake::faker::name::pt_br::Name;
use fake::faker::{self, number};
use fake::{Fake, Faker};
use rand::Rng;
use uuid::Uuid;

use crate::dominio::identidade::entidades::usuario::builder::UsuarioBuilder;
use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::utils::sqlx::{DbDateTime, db_date_time_now};

#[derive(Default)]
pub struct UsuarioParcial {
    pub id: Option<Uuid>,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha_hash: Option<String>,
    pub url_curriculo_lattes: Option<String>,
    pub registrado_em: Option<DbDateTime>,
    pub atualizado_em: Option<DbDateTime>,
    pub desativado_em: Option<DbDateTime>,
}

impl UsuarioParcial {
    pub fn into_builder(self) -> UsuarioBuilder {
        UsuarioBuilder {
            atualizado_em: self.atualizado_em,
            desativado_em: self.desativado_em,
            email: self.email.unwrap_or_else(|| FreeEmail().fake()),
            id: self.id.unwrap_or_else(Uuid::new_v4),
            nome: self
                .nome
                .unwrap_or_else(|| faker::name::pt_br::Name().fake()),
            registrado_em: self.registrado_em.unwrap_or_else(db_date_time_now),
            senha_hash: self.senha_hash.unwrap_or_else(|| Password(2..15).fake()),
            url_curriculo_lattes: self.url_curriculo_lattes,
        }
    }

    pub fn into_entidade(self) -> Usuario { self.into_builder().into() }
}

#[derive(Default)]
pub struct UsuarioModeloParcial {
    pub id: Option<Uuid>,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha_hash: Option<String>,
    pub url_curriculo_lattes: Option<String>,
    pub cargo: Option<Cargo>,
    pub registrado_em: Option<DbDateTime>,
    pub atualizado_em: Option<DbDateTime>,
    pub desativado_em: Option<DbDateTime>,
    pub registro_aluno: Option<String>,
    pub periodo: Option<i16>,
}

impl UsuarioModeloParcial {
    pub fn aluno() -> Self {
        Self {
            cargo: Some(Cargo::Aluno),
            // garante que nunca haverá um registro de aluno "a0000001"
            registro_aluno: Some(format!("a{:07}", rand::rng().random_range(2..=9999999))),
            periodo: Some(2),
            ..Default::default()
        }
    }

    pub fn into_entidade(self) -> UsuarioModelo {
        UsuarioModelo {
            id: self.id.unwrap_or(Uuid::new_v4()),
            cargo: self.cargo.unwrap_or(Cargo::Professor),
            email: self.email.unwrap_or_else(|| FreeEmail().fake()),
            nome: self.nome.unwrap_or_else(|| Name().fake()),
            periodo: self.periodo,
            registro_aluno: self.registro_aluno,
            senha_hash: self.senha_hash.unwrap_or_else(|| Password(10..15).fake()),
            url_curriculo_lattes: self.url_curriculo_lattes,
            registrado_em: self.registrado_em.unwrap_or_else(db_date_time_now),
            atualizado_em: self.atualizado_em,
            desativado_em: self.desativado_em,
        }
    }
}
