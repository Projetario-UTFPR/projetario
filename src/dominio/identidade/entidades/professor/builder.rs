use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::builder::UsuarioBuilder;
use crate::dominio::identidade::enums::cargo::Cargo;

pub struct ProfessorBuilder {
    pub usuario: UsuarioBuilder,
    pub cargo: Cargo,
}

impl From<ProfessorBuilder> for Professor {
    fn from(value: ProfessorBuilder) -> Self {
        Self {
            cargo: value.cargo,
            usuario: value.usuario.into(),
        }
    }
}
