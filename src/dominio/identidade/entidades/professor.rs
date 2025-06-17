use sqlx::prelude::FromRow;

use crate::dominio::identidade::{
    entidades::usuario::{Usuario, UsuarioModelo},
    enums::cargo::Cargo,
};

#[derive(Debug, FromRow)]
pub struct Professor {
    #[sqlx(flatten)]
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

impl TryFrom<&UsuarioModelo> for Professor {
    type Error = anyhow::Error;

    fn try_from(value: &UsuarioModelo) -> Result<Self, Self::Error> {
        if ![Cargo::Professor, Cargo::Administrador].contains(&value.cargo) {
            return Err(anyhow::Error::msg(
                "O usuário fornecido não é um professor nem um administrador.",
            ));
        }

        if value.registro_aluno.is_some() {
            log::warn!(
                "Foi encontrado um usuário de cargo {} que possui registro de aluno.",
                &value.cargo
            )
        }

        let usuario = Usuario {
            atualizado_em: value.atualizado_em,
            desativado_em: value.desativado_em,
            email: value.email.to_owned(),
            id: value.id,
            nome: value.nome.to_owned(),
            registrado_em: value.registrado_em,
            senha_hash: value.senha_hash.to_owned(),
            url_curriculo_lates: value.url_curriculo_lates.to_owned(),
        };

        Ok(Self {
            usuario,
            cargo: value.cargo.to_owned(),
        })
    }
}
