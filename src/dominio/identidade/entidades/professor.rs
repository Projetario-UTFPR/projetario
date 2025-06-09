use crate::dominio::identidade::{entidades::usuario::Pessoa, enums::cargo::Cargo};

#[derive(Debug)]
pub struct Professor {
    pessoa: Pessoa,
    cargo: Cargo,
}
