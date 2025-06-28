use sqlx::prelude::FromRow;

use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::identidade::traits::IntoUsuarioModelo;

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
        url_curriculo_lattes: Option<String>,
    ) -> Self {
        Self::novo_com_cargo(
            nome,
            email,
            senha_hash,
            url_curriculo_lattes,
            Cargo::Professor,
        )
    }

    pub fn novo_com_cargo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lattes: Option<String>,
        cargo: Cargo,
    ) -> Self {
        let usuario = Usuario::novo(nome, email, senha_hash, url_curriculo_lattes);
        Self { usuario, cargo }
    }
}

impl Professor {
    pub fn obtenha_usuario(&self) -> &Usuario { self.as_ref() }

    pub fn obtenha_usuario_mutavel(&mut self) -> &mut Usuario { self.as_mut() }

    pub fn coloque_cargo(&mut self, cargo: Cargo) {
        self.cargo = cargo;
        self.usuario.toque();
    }

    pub fn obtenha_cargo(&self) -> &Cargo { &self.cargo }
}

impl AsRef<Usuario> for Professor {
    fn as_ref(&self) -> &Usuario { &self.usuario }
}

impl AsMut<Usuario> for Professor {
    fn as_mut(&mut self) -> &mut Usuario { &mut self.usuario }
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
            url_curriculo_lattes: value.url_curriculo_lattes.to_owned(),
        };

        Ok(Self {
            usuario,
            cargo: value.cargo.to_owned(),
        })
    }
}

impl IntoUsuarioModelo for Professor {
    fn into_usuario_modelo(self) -> UsuarioModelo {
        UsuarioModelo {
            id: self.usuario.id,
            nome: self.usuario.nome,
            email: self.usuario.email,
            senha_hash: self.usuario.senha_hash,
            cargo: self.cargo,
            url_curriculo_lattes: self.usuario.url_curriculo_lattes,
            registrado_em: self.usuario.registrado_em,
            atualizado_em: self.usuario.atualizado_em,
            desativado_em: self.usuario.desativado_em,
            registro_aluno: None,
            periodo: None,
        }
    }
}
