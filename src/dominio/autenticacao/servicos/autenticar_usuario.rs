use std::borrow::Cow;
use std::hash::Hash;

use crate::dominio::autenticacao::{ComparadorDeHashDeSenha, HasherDeSenha};
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::dominio::identidade::traits::IntoUsuarioModelo;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
use crate::utils::erros::resultado_de_dominio::ResultadoDominio;

pub enum TipoDeLogin<'this> {
    EmailInstitucional(&'this str),
    RegistroDeAluno(&'this str),
}

pub struct AutenticarUsuarioParams<'this> {
    pub login: TipoDeLogin<'this>,
    pub senha: &'this str,
}

pub enum AutenticarUsuarioResult {
    Autenticado(UsuarioModelo),
    NaoAutenticado,
}

pub struct ServicoAutenticarUsuario<Comparador, RU>
where
    RU: RepositorioDeUsuarios,
    Comparador: ComparadorDeHashDeSenha,
{
    comparador: Comparador,
    repositorio_usuarios: RU,
}

impl<Comparador, RU> ServicoAutenticarUsuario<Comparador, RU>
where
    RU: RepositorioDeUsuarios,
    Comparador: ComparadorDeHashDeSenha,
{
    pub async fn executar(
        &self,
        params: AutenticarUsuarioParams<'_>,
    ) -> ResultadoDominio<AutenticarUsuarioResult> {
        if let TipoDeLogin::RegistroDeAluno(ra) = &params.login {
            if !ra.starts_with('a') || !ra[1..].chars().all(|char| char.is_numeric()) {
                return Err(ErroDeDominio::valor_invalido(
                    "O registro de aluno deve ser prefixado com a letra \"a\".",
                ));
            }
        }

        let usuario = match params.login {
            TipoDeLogin::EmailInstitucional(email) => self
                .repositorio_usuarios
                .encontre_professor_pelo_email(email.as_ref())
                .await?
                .map(IntoUsuarioModelo::into_usuario_modelo),

            TipoDeLogin::RegistroDeAluno(ra) => self
                .repositorio_usuarios
                .encontre_aluno_pelo_ra(ra.as_ref())
                .await?
                .map(IntoUsuarioModelo::into_usuario_modelo),
        };

        let usuario = match usuario {
            None => return Ok(AutenticarUsuarioResult::NaoAutenticado),
            Some(usuario) => usuario,
        };

        let senha_esta_correta = self
            .comparador
            .compare(params.senha, usuario.senha_hash.as_str());

        if !senha_esta_correta {
            return Ok(AutenticarUsuarioResult::NaoAutenticado);
        }

        Ok(AutenticarUsuarioResult::Autenticado(usuario))
    }
}

// TODO: adicionar testes unitários para verificar se o service está certo
