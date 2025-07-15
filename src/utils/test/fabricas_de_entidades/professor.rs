use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::professor::builder::ProfessorBuilder;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::utils::test::fabricas_de_entidades::usuario_modelo::UsuarioParcial;

#[derive(Default)]
pub struct ProfessorParcial {
    pub usuario: UsuarioParcial,
    pub cargo: Option<Cargo>,
}

impl ProfessorParcial {
    pub fn into_builder(self) -> ProfessorBuilder {
        ProfessorBuilder {
            cargo: self.cargo.unwrap_or(Cargo::Professor),
            usuario: self.usuario.into_builder(),
        }
    }

    pub fn into_entidade(self) -> Professor { self.into_builder().into() }
}
