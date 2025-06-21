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

#[derive(Debug)]
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
    pub fn novo(comparador: Comparador, repositorio_usuarios: RU) -> Self {
        Self {
            comparador,
            repositorio_usuarios,
        }
    }

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

#[cfg(test)]
mod test {
    use std::result;

    use rstest::rstest;

    use crate::dominio::autenticacao::HasherDeSenha;
    use crate::dominio::autenticacao::servicos::autenticar_usuario::{
        AutenticarUsuarioParams,
        AutenticarUsuarioResult,
        ServicoAutenticarUsuario,
        TipoDeLogin,
    };
    use crate::utils::test::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaFake;
    use crate::utils::test::fabricas_de_entidades::usuario_modelo::{
        FabricaUsuarioModelo,
        UsuarioModeloConstrutor,
    };
    use crate::utils::test::repositorios_em_memoria::fabricas::fabrica_repositorio_de_usuarios::FabricaRepositorioDeUsuarios;
    use crate::utils::test::repositorios_em_memoria::usuarios::RepositorioDeUsuariosEmMemoria;

    type Sut =
        ServicoAutenticarUsuario<ComparadorEHasherDeSenhaFake, RepositorioDeUsuariosEmMemoria>;

    #[rstest::fixture]
    fn sut() -> Sut {
        let repositorio_usuarios = FabricaRepositorioDeUsuarios::obtenha_repositorio();
        let comparador = ComparadorEHasherDeSenhaFake::novo();

        let mut professor = UsuarioModeloConstrutor {
            email: Some("professor@utfpr.edu.br".into()),
            senha_hash: Some(comparador.aplique_hash("123456")),
            ..Default::default()
        };

        repositorio_usuarios
            .usuarios_tbl
            .lock()
            .unwrap()
            .push(FabricaUsuarioModelo::obtenha_entidade(professor));

        let mut aluno = UsuarioModeloConstrutor::aluno();
        aluno.registro_aluno = Some("a2600554".into());
        aluno.senha_hash = Some(comparador.aplique_hash("123456"));

        repositorio_usuarios
            .usuarios_tbl
            .lock()
            .unwrap()
            .push(FabricaUsuarioModelo::obtenha_entidade(aluno));

        ServicoAutenticarUsuario::novo(comparador, repositorio_usuarios)
    }

    #[rstest]
    #[tokio::test]
    async fn deveria_rejeitar_registro_de_aluno_que_nao_inicia_com_a_letra_a(sut: Sut) {
        let resultado = sut
            .executar(AutenticarUsuarioParams {
                login: TipoDeLogin::RegistroDeAluno("2600445"),
                senha: "foo",
            })
            .await;

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().mensagem().contains("letra \"a\""));
    }

    #[rstest]
    #[tokio::test]
    async fn deveria_autenticar_um_usuario_valido(sut: Sut) {
        let resultado = sut
            .executar(AutenticarUsuarioParams {
                login: TipoDeLogin::RegistroDeAluno("a2600554"),
                senha: "123456",
            })
            .await;

        assert!(resultado.is_ok());

        let resultado = resultado.unwrap();
        assert!(matches!(resultado, AutenticarUsuarioResult::Autenticado(_)));
    }

    #[rstest]
    #[tokio::test]
    async fn deveria_autenticar_um_professor(sut: Sut) {
        let resultado = sut
            .executar(AutenticarUsuarioParams {
                login: TipoDeLogin::EmailInstitucional("professor@utfpr.edu.br"),
                senha: "123456",
            })
            .await;

        assert!(resultado.is_ok());

        assert!(matches!(
            resultado.unwrap(),
            AutenticarUsuarioResult::Autenticado(_)
        ));
    }

    #[rstest]
    #[case(TipoDeLogin::EmailInstitucional("foo@gmail.com"))]
    #[case(TipoDeLogin::RegistroDeAluno("a0000001"))]
    #[tokio::test]
    async fn deveria_informar_nao_autenticacao_quando_o_usuario_nao_eh_encontrado(
        #[case] login: TipoDeLogin<'_>,
        sut: Sut,
    ) {
        let resultado = sut
            .executar(AutenticarUsuarioParams {
                login,
                senha: "123",
            })
            .await;

        assert!(resultado.is_ok());
        assert!(matches!(
            resultado.unwrap(),
            AutenticarUsuarioResult::NaoAutenticado
        ));
    }

    async fn deveria_informar_nao_autenticacao_quando_a_senha_esta_errada(sut: Sut) {
        let resultado = sut
            .executar(AutenticarUsuarioParams {
                login: TipoDeLogin::EmailInstitucional("professor@utfpr.edu.br"),
                senha: "123",
            })
            .await;

        assert!(resultado.is_ok());
        assert!(matches!(
            resultado.unwrap(),
            AutenticarUsuarioResult::NaoAutenticado
        ));
    }
}
