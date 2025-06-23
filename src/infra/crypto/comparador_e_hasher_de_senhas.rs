use hmac::digest::generic_array::GenericArray;
use hmac::digest::{Key, KeyInit, MacError};
use hmac::{Hmac, Mac, SimpleHmac};
use openssl::base64;
use sha2::Sha256;

use crate::dominio::autenticacao::{
    ComparadorDeHashDeSenha,
    ComparadorEHasherDeSenha,
    HasherDeSenha,
};

type HmacSha256 = Hmac<Sha256>;

pub struct ComparadorEHasherDeSenhasCrypto;

impl ComparadorEHasherDeSenhasCrypto {
    pub fn novo() -> Self { Self }
}

impl HasherDeSenha for ComparadorEHasherDeSenhasCrypto {
    fn aplique_hash(&self, senha_crua: &str) -> String {
        let rng = hmac::digest::crypto_common::rand_core::OsRng;
        let salt = HmacSha256::generate_key(rng);

        let mut mac: HmacSha256 = Mac::new(&salt);
        mac.update(senha_crua.as_bytes());
        let bytes_do_hash = mac.finalize();

        let senha_hasheada = format!(
            "{}-{}",
            base64::encode_block(bytes_do_hash.into_bytes().as_slice()),
            base64::encode_block(salt.as_slice())
        );

        senha_hasheada
    }
}

impl ComparadorDeHashDeSenha for ComparadorEHasherDeSenhasCrypto {
    fn compare(&self, senha_plana: &str, hash: &str) -> bool {
        let partes = hash.split('-').collect::<Vec<&str>>();

        if partes.len() != 2 {
            return false;
        }

        let hash = partes[0];
        let salt = partes[1];

        let salt_como_bytes = match base64::decode_block(salt) {
            Err(_) => return false,
            Ok(bytes) => bytes,
        };

        let salt = Key::<HmacSha256>::from_slice(&salt_como_bytes);
        let mut mac: HmacSha256 = Mac::new(salt);

        mac.update(senha_plana.as_bytes());

        let hash_como_bytes = match base64::decode_block(hash) {
            Err(_) => return false,
            Ok(bytes) => bytes,
        };

        mac.verify_slice(&hash_como_bytes).is_ok()
    }
}

impl ComparadorEHasherDeSenha for ComparadorEHasherDeSenhasCrypto {}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use crate::dominio::autenticacao::{ComparadorDeHashDeSenha, HasherDeSenha};
    use crate::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhasCrypto;

    #[test]
    fn deveria_retornar_true_se_e_somente_se_as_senhas_coincidem() {
        let hasher = ComparadorEHasherDeSenhasCrypto::novo();

        let senha_em_plain_text = "12345678Aa!";
        let senha_hash = hasher.aplique_hash(senha_em_plain_text);

        assert!(hasher.compare(senha_em_plain_text, &senha_hash));
        assert!(!hasher.compare("12345678aa!", &senha_hash));
        assert!(!hasher.compare("12345678aA!", &senha_hash));
        assert!(!hasher.compare("12345678Aa", &senha_hash));
        assert!(!hasher.compare("22345678Aa!", &senha_hash));
        assert!(!hasher.compare("svjkhvdkshj9&*@¨¨$$T&JHBvjhdsb", &senha_hash));
    }
}
