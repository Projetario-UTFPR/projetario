use std::sync::{Arc, Mutex};

use crate::{
    dominio::projetos::entidades::projeto::Projeto,
    utils::test::repositorios_em_memoria::{
        TabelaThreadSafeEmMemoria,
        coordenadores_de_projetos::{
            ProjetoCoordenadorTupla, RepositorioDeCoordenadoresDeProjetosEmMemoria,
        },
    },
};

pub struct FabricaRepositorioDeCoordenadoresDeProjetos;

impl FabricaRepositorioDeCoordenadoresDeProjetos {
    pub fn obtenha_repositorio() -> RepositorioDeCoordenadoresDeProjetosEmMemoria {
        RepositorioDeCoordenadoresDeProjetosEmMemoria {
            projeto_tbl: Arc::new(Mutex::new(Vec::new())),
            projeto_coordenador_tbl: Arc::new(Mutex::new(Vec::new())),
        }
    }

    // Será necessário futuramente.
    // pub fn obtenha_repositorio_com_tabelas(
    //     projeto_tbl: TabelaThreadSafeEmMemoria<Projeto>,
    //     projeto_coordenador_tbl: TabelaThreadSafeEmMemoria<ProjetoCoordenadorTupla>,
    // ) -> RepositorioDeCoordenadoresDeProjetosEmMemoria {
    //     RepositorioDeCoordenadoresDeProjetosEmMemoria {
    //         projeto_tbl,
    //         projeto_coordenador_tbl,
    //     }
    // }
}
