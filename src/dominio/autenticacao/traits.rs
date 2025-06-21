use std::borrow::Cow;

pub trait HasherDeSenha {
    fn aplique_hash(&self, senha_crua: &str) -> String;
}

pub trait ComparadorDeHashDeSenha {
    fn compare(&self, senha_crua: &str, hash: &str) -> bool;
}

pub trait ComparadorEHasherDeSenha: HasherDeSenha + ComparadorDeHashDeSenha {}
