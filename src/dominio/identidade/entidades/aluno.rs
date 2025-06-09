use uuid::Uuid;

use crate::dominio::identidade::entidades::usuario::Usuario;

#[derive(Debug)]
pub struct Aluno {
    usuario: Usuario,
    registro_aluno: String,
    // o tipo `u8` seria preferível, mas não pode ser obtido de uma resposta do postgres,
    // cujo menor número é o i16.
    periodo: i16,
}
