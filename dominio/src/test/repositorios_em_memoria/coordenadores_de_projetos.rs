use async_trait::async_trait;
use comum::erros::ResultadoDominio;
use comum::erros::erro_de_dominio::ErroDeDominio;
use uuid::Uuid;

use crate::identidade::entidades::professor::Professor;
use crate::identidade::entidades::usuario::UsuarioModelo;
use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
use crate::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;

pub struct ProjetoCoordenadorTupla {
    pub id_professor: Uuid,
    pub id_projeto: Uuid,
}

#[derive(Clone)]
pub struct RepositorioDeCoordenadoresDeProjetosEmMemoria {
    pub usuarios_tbl: TabelaThreadSafeEmMemoria<UsuarioModelo>,
    pub projeto_tbl: TabelaThreadSafeEmMemoria<Projeto>,
    pub projeto_coordenador_tbl: TabelaThreadSafeEmMemoria<ProjetoCoordenadorTupla>,
}

#[async_trait]
impl RepositorioDeCoordenadoresDeProjetos for RepositorioDeCoordenadoresDeProjetosEmMemoria {
    async fn criar_projeto_com_coordenador(
        &self,
        projeto: &Projeto,
        coordenador: &Professor,
    ) -> Result<(), ErroDeDominio> {
        self.projeto_tbl.lock().unwrap().push(projeto.clone());

        self.projeto_coordenador_tbl
            .lock()
            .unwrap()
            .push(ProjetoCoordenadorTupla {
                id_professor: *coordenador.obtenha_usuario().obtenha_id(),
                id_projeto: *projeto.obtenha_id(),
            });

        Ok(())
    }

    async fn buscar_projeto_e_coordenadores_por_id(
        &self,
        id_projeto: &Uuid,
    ) -> ResultadoDominio<Option<ProjetoComCoordenadores>> {
        let projeto = self
            .projeto_tbl
            .lock()
            .expect("Projeto não encontrado no ambiente de testes.")
            .iter()
            .find(|projeto| projeto.obtenha_id().eq(id_projeto))
            .cloned();

        let projeto = match projeto {
            None => return Ok(None),
            Some(p) => p,
        };

        let coord = self
            .projeto_coordenador_tbl
            .lock()
            .unwrap()
            .iter()
            .find(|rel| rel.id_projeto.eq(id_projeto))
            .map(|rel| &rel.id_professor)
            .cloned();

        let coord = match coord {
            None => None,
            Some(id) => self
                .usuarios_tbl
                .lock()
                .expect("Coordenador não encontrado no ambiente de testes.")
                .iter()
                .find(|usuario| usuario.id.eq(&id))
                .cloned(),
        }
        .map(|coord| Professor::try_from(&coord).expect("Um não-professor foi inserido na tabela de relacionamento projeto-coordenador de testes."))
        .unwrap();

        Ok(Some(ProjetoComCoordenadores::novo(
            projeto.clone(),
            coord,
            None,
        )))
    }
}
