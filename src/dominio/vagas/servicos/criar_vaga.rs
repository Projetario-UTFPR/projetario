use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::ResultadoDominio;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
use crate::utils::sqlx::DbDateTime;

pub struct CriarVagaParams<'this> {
    pub professor: &'this Professor,
    pub id_projeto: Uuid,
    pub horas_por_semana: u8,
    pub imagem: String,
    pub quantidade: u8,
    pub link_edital: String,
    pub conteudo: String,
    pub titulo: Option<String>,
    pub link_candidatura: Option<String>,
    pub inscricoes_ate: DbDateTime,
}

pub struct ServicoCriarVaga<RV, RPC>
where
    RV: RepositorioDeVagas,
    RPC: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_vagas: RV,
    repositorio_de_projetos_e_coords: RPC,
}

impl<RV, RPC> ServicoCriarVaga<RV, RPC>
where
    RV: RepositorioDeVagas,
    RPC: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_vagas: RV, repositorio_de_projetos_e_coords: RPC) -> Self {
        Self {
            repositorio_de_vagas,
            repositorio_de_projetos_e_coords,
        }
    }

    pub async fn executar(
        &self,
        CriarVagaParams {
            conteudo,
            professor,
            horas_por_semana,
            id_projeto,
            imagem,
            inscricoes_ate,
            link_candidatura,
            link_edital,
            quantidade,
            titulo,
        }: CriarVagaParams<'_>,
    ) -> ResultadoDominio<Vaga> {
        let projeto_com_coords = self
            .repositorio_de_projetos_e_coords
            .buscar_projeto_e_coordenadores_por_id(&id_projeto)
            .await?;

        let (projeto, coordenador, vice_coordenador) = projeto_com_coords
            .ok_or_else(|| ErroDeDominio::nao_encontrado("O projeto informado não foi encontrado."))
            .map(|projeto| projeto.desestruture())?;

        if !projeto.esta_ativo() {
            return Err(ErroDeDominio::integridade(
                "Não é permitido abrir vagas para um projeto desativado.",
            ));
        }

        if Cargo::Administrador.ne(professor.obtenha_cargo()) && coordenador.ne(professor) {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem autorização para abrir vagas para este projeto.",
            ));
        }

        let vaga = Vaga::nova(
            projeto,
            coordenador,
            vice_coordenador,
            horas_por_semana,
            imagem,
            quantidade,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            inscricoes_ate,
        )?;

        self.repositorio_de_vagas.criar_vaga(&vaga).await?;

        Ok(vaga)
    }
}

#[cfg(test)]
mod test {
    use std::result;

    use chrono::Utc;
    use rstest::fixture;
    use uuid::Uuid;

    use crate::{dominio::{identidade::{entidades::{professor::Professor, usuario::UsuarioModelo}, traits::IntoUsuarioModelo}, vagas::servicos::criar_vaga::{CriarVagaParams, ServicoCriarVaga}}, utils::{sqlx::db_date_time_now, test::{fabricas_de_entidades::usuario_modelo::{FabricaUsuarioModelo, UsuarioModeloParcial}, repositorios_em_memoria::{coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosEmMemoria, fabricas::{fabrica_repositorio_de_coordenadores_de_projetos::{self, FabricaRepositorioDeCoordenadoresDeProjetos}, fabrica_repositorio_de_usuarios::FabricaRepositorioDeUsuarios, fabrica_repositorio_de_vagas::FabricaRepositorioDeVagas}, usuarios::RepositorioDeUsuariosEmMemoria, vagas::RepositorioDeVagasEmMemoria}}}};

    #[tokio::test]
    async fn nao_deveria_criar_vaga_para_um_projeto_inexistente() {
        let ServicoERepos {
            sut,
            repo_de_usuarios,
            ..
        } = obtehna_servico_e_repos();
        let professor =
            Professor::novo("John Doe".into(), "john@gmail.com".into(), "".into(), None);
        repo_de_usuarios
            .usuarios_tbl
            .lock()
            .unwrap()
            .push(professor.clone().into_usuario_modelo());

        let resultado = sut
            .executar(CriarVagaParams {
                conteudo: "Foo".into(),
                horas_por_semana: 20,
                id_projeto: Uuid::new_v4(),
                imagem: "aa".into(),
                inscricoes_ate: db_date_time_now(),
                link_candidatura: None,
                link_edital: "".into(),
                professor: &professor,
                quantidade: 2,
                titulo: None,
            })
            .await;

        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn nao_deveria_criar_vagas_pra_um_projeto_desativado() { todo!() }

    #[tokio::test]
    async fn somente_o_coordenador_ou_um_administrador_devem_poder_abrir_vagas_para_um_projeto() {
        todo!()
    }

    #[tokio::test]
    async fn deveria_criar_vaga_para_um_projeto_regular() { todo!() }

    fn obtehna_servico_e_repos() -> ServicoERepos {
        let mut repo_de_projetos =
            FabricaRepositorioDeCoordenadoresDeProjetos::obtenha_repositorio();
        let mut repo_de_usuarios = FabricaRepositorioDeUsuarios::obtenha_repositorio();
        repo_de_projetos.usuarios_tbl = repo_de_projetos.usuarios_tbl.clone();
        let mut repo_de_vagas = FabricaRepositorioDeVagas::obtenha_repositorio();

        {
            let mut tbl = repo_de_usuarios.usuarios_tbl.lock().unwrap();
            tbl.push(FabricaUsuarioModelo::obtenha_entidade(
                UsuarioModeloParcial::default(),
            ));
        }

        ServicoERepos {
            sut: ServicoCriarVaga::novo(repo_de_vagas.clone(), repo_de_projetos.clone()),
            repo_de_proj_e_coords: repo_de_projetos,
            repo_de_usuarios,
            repo_de_vagas,
        }
    }

    struct ServicoERepos {
        pub sut: ServicoCriarVaga<
            RepositorioDeVagasEmMemoria,
            RepositorioDeCoordenadoresDeProjetosEmMemoria,
        >,
        pub repo_de_vagas: RepositorioDeVagasEmMemoria,
        pub repo_de_proj_e_coords: RepositorioDeCoordenadoresDeProjetosEmMemoria,
        pub repo_de_usuarios: RepositorioDeUsuariosEmMemoria,
    }
}
