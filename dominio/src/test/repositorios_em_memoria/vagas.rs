use async_trait::async_trait;
use comum::erros::{ErroDeDominio, ResultadoDominio};
use uuid::Uuid;

use crate::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::repositorios::vaga::RepositorioDeVagas;

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
