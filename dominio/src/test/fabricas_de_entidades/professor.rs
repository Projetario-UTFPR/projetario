use crate::identidade::entidades::professor::{Professor, ProfessorBuilder};
use crate::identidade::enums::cargo::Cargo;
use crate::test::fabricas_de_entidades::usuario_modelo::UsuarioParcial;

#[derive(Default)]
pub struct ProfessorParcial {
    pub usuario: UsuarioParcial,
    pub cargo: Option<Cargo>,
}

impl ProfessorParcial {
    pub fn into_builder(self) -> ProfessorBuilder {
        let mut builder = ProfessorBuilder::default();

        builder
            .cargo(self.cargo.unwrap_or(Cargo::Professor))
            .usuario(self.usuario.into_entidade());

        builder
    }

    pub fn into_entidade(self) -> Professor { self.into_builder().build().unwrap() }
}
