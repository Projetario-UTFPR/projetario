use crate::dominio::identidade::enums::cargo::Cargo;

pub struct PoliticasDeAutorizacao;

impl PoliticasDeAutorizacao {
    pub fn hierarquia_do_cargo_permite(cargo: &Cargo, expectativa: &Cargo) -> bool {
        match expectativa {
            Cargo::Aluno => cargo.eq(expectativa),
            Cargo::Administrador => cargo.eq(expectativa),
            Cargo::Professor => [Cargo::Administrador, Cargo::Professor].contains(cargo),
        }
    }
}
