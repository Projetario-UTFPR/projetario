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

impl Aluno {
    pub fn obtenha_usuario(&self) -> &Usuario {
        self.as_ref()
    }

    pub fn obtenha_usuario_mutavel(&mut self) -> &mut Usuario {
        self.as_mut()
    }

    pub fn obtenha_registro_de_aluno(&mut self) -> &str {
        &self.registro_aluno
    }

    pub fn obtenha_periodo(&self) -> i16 {
        self.periodo
    }

    pub fn coloque_periodo(&mut self, periodo: i16) {
        self.periodo = periodo;
        self.usuario.toque();
    }
}

impl AsRef<Usuario> for Aluno {
    fn as_ref(&self) -> &Usuario {
        &self.usuario
    }
}

impl AsMut<Usuario> for Aluno {
    fn as_mut(&mut self) -> &mut Usuario {
        &mut self.usuario
    }
}
