use openssl::base64;
use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use crate::dominio::autenticacao::{
    ComparadorDeHashDeSenha,
    ComparadorEHasherDeSenha,
    HasherDeSenha,
};
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};

pub struct ComparadorEHasherDeSenhaCrypto;

impl ComparadorEHasherDeSenhaCrypto {
    pub fn novo() -> Self { Self }
}

impl HasherDeSenha for ComparadorEHasherDeSenhaCrypto {
    fn aplique_hash(&self, senha_crua: &str) -> ResultadoDominio<String> {
        let salt = SaltString::generate(&mut OsRng);

        let senha_hasheada = match Pbkdf2.hash_password(senha_crua.as_bytes(), &salt) {
            Err(erro) => {
                log::error!(
                    "Houve uma falha no módulo Pbkdf2 ao aplicar hash em uma senha: {erro}"
                );

                return Err(ErroDeDominio::interno());
            }

            Ok(hash) => hash.to_string(),
        };

        Ok(senha_hasheada)
    }
}

impl ComparadorDeHashDeSenha for ComparadorEHasherDeSenhaCrypto {
    fn compare(&self, senha_plana: &str, hash: &str) -> bool {
        let hash = match PasswordHash::new(hash) {
            Err(_) => return false,
            Ok(hash) => hash,
        };

        Pbkdf2
            .verify_password(senha_plana.as_bytes(), &hash)
            .is_ok()
    }
}

impl ComparadorEHasherDeSenha for ComparadorEHasherDeSenhaCrypto {}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::dominio::autenticacao::{ComparadorDeHashDeSenha, HasherDeSenha};
    use crate::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;

    #[test]
    fn deveria_retornar_true_se_e_somente_se_as_senhas_coincidem() {
        let hasher = ComparadorEHasherDeSenhaCrypto::novo();

        let senha_em_plain_text = "12345678Aa!";
        let senha_hash = hasher.aplique_hash(senha_em_plain_text).unwrap();

        assert!(hasher.compare(senha_em_plain_text, &senha_hash));
        assert!(!hasher.compare("12345678aa!", &senha_hash));
        assert!(!hasher.compare("12345678aA!", &senha_hash));
        assert!(!hasher.compare("12345678Aa", &senha_hash));
        assert!(!hasher.compare("22345678Aa!", &senha_hash));
        assert!(!hasher.compare("svjkhvdkshj9&*@¨¨$$T&JHBvjhdsb", &senha_hash));
    }
}
