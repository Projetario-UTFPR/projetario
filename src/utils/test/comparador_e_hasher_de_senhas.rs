use crate::dominio::autenticacao::{
    ComparadorDeHashDeSenha,
    ComparadorEHasherDeSenha,
    HasherDeSenha,
};
use crate::utils::erros::ResultadoDominio;

pub struct ComparadorEHasherDeSenhaFake {
    sufix: &'static str,
}

impl ComparadorEHasherDeSenhaFake {
    pub fn novo() -> Self {
        Self {
            sufix: "--fake-hash",
        }
    }
}

impl ComparadorDeHashDeSenha for ComparadorEHasherDeSenhaFake {
    fn compare(&self, senha_crua: &str, hash: &str) -> bool {
        hash.strip_suffix(self.sufix)
            .map(|original| original.eq(senha_crua))
            .unwrap_or(false)
    }
}

impl HasherDeSenha for ComparadorEHasherDeSenhaFake {
    fn aplique_hash(&self, senha_crua: &str) -> ResultadoDominio<String> {
        Ok(format!("{senha_crua}{}", self.sufix))
    }
}
