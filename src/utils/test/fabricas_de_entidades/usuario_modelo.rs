use chrono::NaiveDateTime;
use fake::faker::chrono::pt_br::DateTime;
use fake::faker::internet::pt_br::{FreeEmail, Password};
use fake::faker::name::pt_br::Name;
use fake::{Fake, Faker};
use uuid::Uuid;

use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;

pub struct FabricaUsuarioModelo;

#[derive(Default)]
pub struct UsuarioModeloConstrutor {
    pub id: Option<Uuid>,
    pub nome: Option<String>,
    pub email: Option<String>,
    pub senha_hash: Option<String>,
    pub url_curriculo_lates: Option<String>,
    pub cargo: Option<Cargo>,
    pub registrado_em: Option<NaiveDateTime>,
    pub atualizado_em: Option<NaiveDateTime>,
    pub desativado_em: Option<NaiveDateTime>,
    pub registro_aluno: Option<String>,
    pub periodo: Option<i16>,
}

impl FabricaUsuarioModelo {
    pub fn obtenha_entidade(parcial: UsuarioModeloConstrutor) -> UsuarioModelo {
        UsuarioModelo {
            id: parcial.id.unwrap_or(Uuid::new_v4()),
            cargo: parcial.cargo.unwrap_or(Cargo::Aluno),
            email: parcial.email.unwrap_or_else(|| FreeEmail().fake()),
            nome: parcial.nome.unwrap_or_else(|| Name().fake()),
            periodo: parcial.periodo,
            registro_aluno: parcial.registro_aluno,
            senha_hash: parcial
                .senha_hash
                .unwrap_or_else(|| Password(10..15).fake()),
            url_curriculo_lates: parcial.url_curriculo_lates,
            registrado_em: parcial.registrado_em.unwrap_or_else(|| DateTime().fake()),
            atualizado_em: parcial.atualizado_em,
            desativado_em: parcial.desativado_em,
        }
    }
}
