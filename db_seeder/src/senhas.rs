use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::rand_core::OsRng;
use pbkdf2::password_hash::{PasswordHasher, SaltString};

pub fn aplicar_hash(senha: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);

    Pbkdf2
        .hash_password(senha.as_bytes(), &salt)
        .expect("Não foi possível aplicar hash à senha do usuário mockado durante o seed.")
        .to_string()
}
