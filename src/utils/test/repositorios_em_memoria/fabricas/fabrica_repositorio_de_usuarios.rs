use std::sync::{Arc, Mutex};

use crate::utils::test::repositorios_em_memoria::usuarios::RepositorioDeUsuariosEmMemoria;

pub struct FabricaRepositorioDeUsuarios;

impl FabricaRepositorioDeUsuarios {
    pub fn obtenha_repositorio() -> RepositorioDeUsuariosEmMemoria {
        RepositorioDeUsuariosEmMemoria {
            usuarios_tbl: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
