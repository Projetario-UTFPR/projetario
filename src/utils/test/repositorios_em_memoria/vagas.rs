use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::{ErroDeDominio, ResultadoDominio};
use crate::utils::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;

#[derive(Clone)]
pub struct RepositorioDeVagasEmMemoria {
    pub vagas_tbl: TabelaThreadSafeEmMemoria<Vaga>,
}

#[async_trait]
impl RepositorioDeVagas for RepositorioDeVagasEmMemoria {
    async fn criar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()> {
        self.vagas_tbl.lock().unwrap().push(vaga.clone());
        Ok(())
    }

    async fn buscar_por_id(&self, id: &Uuid) -> ResultadoDominio<Option<Vaga>> {
        Ok(self
            .vagas_tbl
            .lock()
            .unwrap()
            .iter()
            .find(|vaga| vaga.obtenha_id().eq(id))
            .cloned())
    }

    async fn atualizar_vaga(&self, vaga: &Vaga) -> ResultadoDominio<()> {
        let mut tbl = self.vagas_tbl.lock().unwrap();

        let index = tbl
            .iter()
            .position(|_vaga| _vaga.obtenha_id().eq(vaga.obtenha_id()));

        let index = match index {
            None => return Err(ErroDeDominio::nao_encontrado("Vaga não encontrada.")),
            Some(index) => index,
        };

        *tbl.get_mut(index).unwrap() = vaga.clone();
        Ok(())
    }
}
