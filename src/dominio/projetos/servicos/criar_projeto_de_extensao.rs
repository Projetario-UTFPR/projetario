use chrono::NaiveDate;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
pub struct CriarProjetosDeExtensaoParams<'a> {
    pub professor: &'a Professor,
    pub titulo: String,
    pub descricao: String,
    pub data_de_inicio: Option<NaiveDate>,
}

pub struct ServicoCriarProjetoDeExtensao<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_coordenadores: RCP,
}

impl<RCP> ServicoCriarProjetoDeExtensao<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_coordenadores: RCP) -> Self {
        Self {
            repositorio_de_coordenadores,
        }
    }

    /// Salva o `projeto` no banco de dados e **imediatamente associa o `usuario` como
    /// coordenador do projeto**, se este for autorizado a coordenar projetos.
    pub async fn executar(
        &self,
        params: CriarProjetosDeExtensaoParams<'_>,
    ) -> Result<Projeto, ErroDeDominio> {
        let CriarProjetosDeExtensaoParams {
            professor,
            descricao,
            titulo,
            data_de_inicio,
        } = params;

        let projeto = match data_de_inicio {
            None => Projeto::novo(titulo, descricao, TipoDeProjeto::Extensao),
            Some(data) => {
                Projeto::novo_com_data_de_inicio(titulo, descricao, TipoDeProjeto::Extensao, data)
            }
        };

        self.repositorio_de_coordenadores
            .criar_projeto_com_coordenador(&projeto, professor)
            .await?;

        Ok(projeto)
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use crate::{
        dominio::{
            identidade::{entidades::usuario::UsuarioModelo, enums::cargo::Cargo},
            projetos::servicos::criar_projeto_de_extensao::{
                CriarProjetosDeExtensaoParams, ServicoCriarProjetoDeExtensao,
            },
        },
        utils::test::{
            fabricas_de_entidades::usuario_modelo::{
                FabricaUsuarioModelo, UsuarioModeloConstrutor,
            },
            repositorios_em_memoria::fabricas::fabrica_repositorio_de_coordenadores_de_projetos::FabricaRepositorioDeCoordenadoresDeProjetos,
        },
    };

    #[tokio::test]
    #[rstest]
    #[case(Cargo::Professor)]
    #[case(Cargo::Administrador)]
    async fn deveria_criar_um_projeto_se_usuario_tiver_autorizacao(#[case] cargo: Cargo) {
        let repositorio_de_coordenadores =
            FabricaRepositorioDeCoordenadoresDeProjetos::obtenha_repositorio();

        let projeto_tbl = repositorio_de_coordenadores.projeto_tbl.clone();
        let relacionamento_tbl = repositorio_de_coordenadores.projeto_coordenador_tbl.clone();

        let sut = ServicoCriarProjetoDeExtensao::novo(repositorio_de_coordenadores);

        let usuario_autorizado = FabricaUsuarioModelo::obtenha_entidade(UsuarioModeloConstrutor {
            cargo: Some(cargo),
            ..Default::default()
        });

        let professor = (&usuario_autorizado).try_into().unwrap();

        let titulo = "Proident ex in aliqua in officia exercitation.".to_string();
        let resposta = sut.executar(CriarProjetosDeExtensaoParams {
            data_de_inicio: None,
            descricao: "Laborum sit exercitation incididunt id ullamco laboris ipsum eiusmod proident \
                        occaecat ex. Eu Lorem qui occaecat laboris laboris. Enim do incididunt aliquip est \
                        magna ipsum elit. Elit enim Lorem reprehenderit consequat cillum sunt Lorem cillum \
                        aliqua adipisicing nostrud tempor ipsum Lorem.\n\
                        Eu consequat minim et sunt cillum magna incididunt esse pariatur exercitation in ea. \
                        Id commodo deserunt eu aliquip ut cillum occaecat pariatur deserunt proident. Cillum \
                        officia nulla duis velit elit.".into(),
            titulo: titulo.clone(),
            professor: &professor
        }).await;

        assert!(resposta.is_ok());
        assert_eq!(1, projeto_tbl.lock().unwrap().len());
        assert_eq!(1, relacionamento_tbl.lock().unwrap().len());

        assert_eq!(
            projeto_tbl.lock().unwrap()[0].obtenha_id(),
            &relacionamento_tbl.lock().unwrap()[0].id_projeto
        );

        assert_eq!(titulo, resposta.unwrap().obtenha_titulo());
    }
}
