use crate::dominio::identidade::{entidades::usuario::Usuario, enums::cargo::Cargo};

#[derive(Debug)]
pub struct Professor {
    usuario: Usuario,
    cargo: Cargo,
}
