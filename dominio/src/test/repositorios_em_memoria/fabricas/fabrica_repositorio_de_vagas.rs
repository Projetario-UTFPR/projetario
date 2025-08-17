use std::sync::{Arc, Mutex};

use crate::test::repositorios_em_memoria::vagas::RepositorioDeVagasEmMemoria;

pub struct FabricaRepositorioDeVagas;

impl FabricaRepositorioDeVagas {
    pub fn obtenha_repositorio() -> RepositorioDeVagasEmMemoria {
        RepositorioDeVagasEmMemoria {
            vagas_tbl: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
