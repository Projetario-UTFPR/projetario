use std::borrow::Cow;

use crate::utils::erros::ResultadoDominio;

pub trait HasherDeSenha {
    /// Obtém um hash da senha. Pode retornar um erro em caso de erros internos,
    /// como alguma falha no recurso de aleatoriedade da máquina host do servidor.
    fn aplique_hash(&self, senha_crua: &str) -> ResultadoDominio<String>;
}

pub trait ComparadorDeHashDeSenha {
    fn compare(&self, senha_crua: &str, hash: &str) -> bool;
}

pub trait ComparadorEHasherDeSenha: HasherDeSenha + ComparadorDeHashDeSenha {}
