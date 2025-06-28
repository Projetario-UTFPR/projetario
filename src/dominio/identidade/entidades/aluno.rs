use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::identidade::traits::IntoUsuarioModelo;
use crate::utils::erros::ErroDeDominio;

#[derive(Debug, FromRow)]
pub struct Aluno {
    #[sqlx(flatten)]
    usuario: Usuario,
    registro_aluno: String,
    // o tipo `u8` seria preferível, mas não pode ser obtido de uma resposta do postgres,
    // cujo menor número é o i16.
    periodo: i16,
}

impl Aluno {
    pub fn novo(
        nome: String,
        email: String,
        senha_hash: String,
        url_curriculo_lattes: Option<String>,
        periodo: i16,
        registro_aluno: String,
    ) -> Self {
        let usuario = Usuario::novo(nome, email, senha_hash, url_curriculo_lattes);
        Self {
            periodo,
            registro_aluno,
            usuario,
        }
    }
}

impl Aluno {
    pub fn obtenha_usuario(&self) -> &Usuario { self.as_ref() }

    pub fn obtenha_usuario_mutavel(&mut self) -> &mut Usuario { self.as_mut() }

    pub fn obtenha_registro_de_aluno(&mut self) -> &str { &self.registro_aluno }

    pub fn obtenha_periodo(&self) -> i16 { self.periodo }

    pub fn coloque_periodo(&mut self, periodo: i16) {
        self.periodo = periodo;
        self.usuario.toque();
    }
}

impl AsRef<Usuario> for Aluno {
    fn as_ref(&self) -> &Usuario { &self.usuario }
}

impl AsMut<Usuario> for Aluno {
    fn as_mut(&mut self) -> &mut Usuario { &mut self.usuario }
}

impl IntoUsuarioModelo for Aluno {
    fn into_usuario_modelo(self) -> UsuarioModelo {
        UsuarioModelo {
            id: self.usuario.id,
            nome: self.usuario.nome,
            email: self.usuario.email,
            senha_hash: self.usuario.senha_hash,
            cargo: Cargo::Aluno,
            url_curriculo_lattes: self.usuario.url_curriculo_lattes,
            registrado_em: self.usuario.registrado_em,
            atualizado_em: self.usuario.atualizado_em,
            desativado_em: self.usuario.desativado_em,
            registro_aluno: Some(self.registro_aluno),
            periodo: Some(self.periodo),
        }
    }
}

impl TryFrom<&UsuarioModelo> for Aluno {
    type Error = ErroDeDominio;

    fn try_from(value: &UsuarioModelo) -> Result<Self, Self::Error> {
        if value.cargo != Cargo::Aluno || value.registro_aluno.is_none() || value.periodo.is_none()
        {
            return Err(ErroDeDominio::valor_invalido(
                "O usuário encontrado não é um aluno válido.",
            ));
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

        Ok(Aluno {
            usuario,
            registro_aluno: value.registro_aluno.to_owned().unwrap(),
            periodo: value.periodo.unwrap(),
        })
    }
}
