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
#[cfg(feature = "test-utils")]
mod test {
    use chrono::{Duration, Utc};
    use fake::faker::{self, lorem};
    use fake::{Fake, Faker};
    use rstest::{fixture, rstest};
    use url::Url;
    use uuid::Uuid;

    use crate::dominio::identidade::entidades::professor::Professor;
    use crate::dominio::identidade::enums::cargo::Cargo;
    use crate::dominio::identidade::traits::IntoUsuarioModelo;
    use crate::dominio::projetos::entidades::projeto::Projeto;
    use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
    use crate::dominio::vagas::servicos::criar_vaga::{CriarVagaParams, ServicoCriarVaga};
    use crate::utils::sqlx::{db_date_time_now, DbDateTime};
    use crate::utils::test::fabricas_de_entidades::usuario_modelo::UsuarioModeloParcial;
    use crate::utils::test::repositorios_em_memoria::coordenadores_de_projetos::{ProjetoCoordenadorTupla, RepositorioDeCoordenadoresDeProjetosEmMemoria};
    use crate::utils::test::repositorios_em_memoria::fabricas::fabrica_repositorio_de_coordenadores_de_projetos::FabricaRepositorioDeCoordenadoresDeProjetos;
    use crate::utils::test::repositorios_em_memoria::fabricas::fabrica_repositorio_de_usuarios::FabricaRepositorioDeUsuarios;
    use crate::utils::test::repositorios_em_memoria::fabricas::fabrica_repositorio_de_vagas::FabricaRepositorioDeVagas;
    use crate::utils::test::repositorios_em_memoria::usuarios::RepositorioDeUsuariosEmMemoria;
    use crate::utils::test::repositorios_em_memoria::vagas::RepositorioDeVagasEmMemoria;

    #[tokio::test]
    async fn nao_deveria_criar_vaga_para_um_projeto_inexistente() {
        let ServicoERepos {
            sut,
            repo_de_usuarios,
            ..
        } = obtehna_servico_e_repos();
        let professor = Professor::novo("John Doe".into(), "john@proj.com".into(), "".into(), None);
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
                inscricoes_ate: obtenha_data_no_futuro(),
                link_candidatura: None,
                link_edital: obtenha_url_aleatorio(),
                professor: &professor,
                quantidade: 2,
                titulo: None,
            })
            .await;

        assert!(resultado.is_err());
    }

    #[tokio::test]
    async fn nao_deveria_criar_vagas_pra_um_projeto_desativado() {
        let ServicoERepos {
            sut,
            repo_de_vagas,
            repo_de_proj_e_coords,
            repo_de_usuarios,
        } = obtehna_servico_e_repos();

        let coord = Professor::novo("Joaquim".into(), "joaquim@proj.com".into(), "".into(), None);
        let mut projeto = Projeto::novo("Foo".into(), "Foo Desc".into(), TipoDeProjeto::Extensao);
        projeto.cancelar();

        repo_de_proj_e_coords
            .projeto_coordenador_tbl
            .lock()
            .unwrap()
            .push(ProjetoCoordenadorTupla {
                id_professor: *coord.obtenha_usuario().obtenha_id(),
                id_projeto: *projeto.obtenha_id(),
            });

        repo_de_proj_e_coords
            .projeto_tbl
            .lock()
            .unwrap()
            .push(projeto.clone());

        repo_de_proj_e_coords
            .usuarios_tbl
            .lock()
            .unwrap()
            .push(coord.clone().into_usuario_modelo());

        let resultado = sut
            .executar(CriarVagaParams {
                professor: &coord,
                id_projeto: *projeto.obtenha_id(),
                horas_por_semana: 20,
                imagem: "".into(),
                quantidade: 1,
                link_edital: obtenha_url_aleatorio(),
                conteudo: "".into(),
                titulo: None,
                link_candidatura: None,
                inscricoes_ate: obtenha_data_no_futuro(),
            })
            .await;

        assert!(resultado.is_err());
        assert!(resultado.unwrap_err().mensagem().contains("desativado"));
    }

    #[rstest]
    #[case(Cargo::Administrador, true)]
    #[case(Cargo::Professor, false)]
    #[tokio::test]
    async fn somente_o_coordenador_ou_um_administrador_devem_poder_abrir_vagas_para_um_projeto(
        #[case] cargo: Cargo,
        #[case] deveria_permitir: bool,
    ) {
        let ServicoERepos {
            sut,
            repo_de_proj_e_coords,
            repo_de_usuarios,
            repo_de_vagas,
        } = obtehna_servico_e_repos();

        let coord = UsuarioModeloParcial::default().into_entidade();

        let projeto = Projeto::novo("Projeto".into(), "Desc".into(), TipoDeProjeto::Extensao);

        repo_de_proj_e_coords
            .projeto_tbl
            .lock()
            .unwrap()
            .push(projeto.clone());

        repo_de_proj_e_coords
            .projeto_coordenador_tbl
            .lock()
            .unwrap()
            .push(ProjetoCoordenadorTupla {
                id_professor: coord.id,
                id_projeto: *projeto.obtenha_id(),
            });

        let usuario = UsuarioModeloParcial {
            cargo: Some(cargo),
            ..Default::default()
        }
        .into_entidade();

        {
            let mut tbl = repo_de_usuarios.usuarios_tbl.lock().unwrap();

            tbl.push(coord);
            tbl.push(usuario.clone());
        }

        let resultado = sut
            .executar(CriarVagaParams {
                professor: &Professor::try_from(&usuario).unwrap(),
                id_projeto: *projeto.obtenha_id(),
                horas_por_semana: 20,
                imagem: "".into(),
                quantidade: 2,
                link_edital: obtenha_url_aleatorio(),
                conteudo: lorem::pt_br::Paragraphs(1..4)
                    .fake::<Vec<String>>()
                    .join("\n"),
                titulo: None,
                link_candidatura: None,
                inscricoes_ate: obtenha_data_no_futuro(),
            })
            .await;

        dbg!(&resultado);
        assert_eq!(deveria_permitir, resultado.is_ok());
    }

    #[tokio::test]
    async fn deveria_criar_vaga_para_um_projeto_regular() {
        let ServicoERepos {
            sut,
            repo_de_proj_e_coords,
            repo_de_usuarios,
            repo_de_vagas,
        } = obtehna_servico_e_repos();

        let projeto = Projeto::novo(
            faker::job::pt_br::Title().fake(),
            faker::job::pt_br::Field().fake(),
            TipoDeProjeto::Extensao,
        );

        let coordenador = UsuarioModeloParcial::default().into_entidade();
        let administrador = UsuarioModeloParcial {
            cargo: Some(Cargo::Administrador),
            ..Default::default()
        }
        .into_entidade();

        {
            let mut tbl = repo_de_usuarios.usuarios_tbl.lock().unwrap();

            tbl.push(coordenador.clone());
            tbl.push(administrador.clone());
        }

        repo_de_proj_e_coords
            .projeto_coordenador_tbl
            .lock()
            .unwrap()
            .push(ProjetoCoordenadorTupla {
                id_professor: coordenador.id,
                id_projeto: *projeto.obtenha_id(),
            });

        repo_de_proj_e_coords
            .projeto_tbl
            .lock()
            .unwrap()
            .push(projeto.clone());

        for professor in [&coordenador, &administrador] {
            let resultado = sut
                .executar(CriarVagaParams {
                    professor: &Professor::try_from(professor).unwrap(),
                    id_projeto: *projeto.obtenha_id(),
                    horas_por_semana: 20,
                    imagem: Faker.fake::<Url>().into(),
                    quantidade: 10,
                    link_edital: obtenha_url_aleatorio(),
                    conteudo: lorem::pt_br::Paragraphs(1..3)
                        .fake::<Vec<String>>()
                        .join("\n"),
                    titulo: None,
                    link_candidatura: None,
                    inscricoes_ate: obtenha_data_no_futuro(),
                })
                .await;

            assert!(resultado.is_ok())
        }
    }

    fn obtehna_servico_e_repos() -> ServicoERepos {
        let mut repo_de_projetos =
            FabricaRepositorioDeCoordenadoresDeProjetos::obtenha_repositorio();
        let mut repo_de_usuarios = FabricaRepositorioDeUsuarios::obtenha_repositorio();
        repo_de_projetos.usuarios_tbl = repo_de_projetos.usuarios_tbl.clone();
        let mut repo_de_vagas = FabricaRepositorioDeVagas::obtenha_repositorio();

        repo_de_projetos.usuarios_tbl = repo_de_usuarios.usuarios_tbl.clone();

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

    fn obtenha_url_aleatorio() -> String { Faker.fake::<Url>().to_string() }

    fn obtenha_data_no_futuro() -> DbDateTime { db_date_time_now() + Duration::days(5) }
}
