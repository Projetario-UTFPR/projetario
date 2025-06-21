use std::borrow::Cow;

pub trait HasherDeSenha {
    fn aplique_hash(&self, senha_crua: Cow<'_, &str>) -> String;
}

pub trait ComparadorDeHashDeSenha {
    fn compare(&self, senha_crua: Cow<'_, &str>, hash: Cow<'_, &str>) -> bool;
}

pub trait ComparadorEHasherDeSenha: HasherDeSenha + ComparadorDeHashDeSenha {}
