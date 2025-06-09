use crate::dominio::identidade::{entidades::usuario::Usuario, enums::cargo::Cargo};

#[derive(Debug)]
pub struct Professor {
    usuario: Usuario,
    cargo: Cargo,
}

impl Professor {
    pub fn novo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lates: Option<String>,
    ) -> Self {
        Self::novo_com_cargo(
            nome,
            email,
            senha_hash,
            url_curriculo_lates,
            Cargo::Professor,
        )
    }

    pub fn novo_com_cargo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lates: Option<String>,
        cargo: Cargo,
    ) -> Self {
        let usuario = Usuario::novo(nome, email, senha_hash, url_curriculo_lates);
        Self { usuario, cargo }
    }
}

impl Professor {
    pub fn obtenha_usuario(&self) -> &Usuario {
        self.as_ref()
    }

    pub fn obtenha_usuario_mutavel(&mut self) -> &mut Usuario {
        self.as_mut()
    }

    pub fn coloque_cargo(&mut self, cargo: Cargo) {
        self.cargo = cargo;
        self.usuario.toque();
    }

    pub fn obtenha_cargo(&self) -> &Cargo {
        &self.cargo
    }
}

impl AsRef<Usuario> for Professor {
    fn as_ref(&self) -> &Usuario {
        &self.usuario
    }
}

impl AsMut<Usuario> for Professor {
    fn as_mut(&mut self) -> &mut Usuario {
        &mut self.usuario
    }
}
