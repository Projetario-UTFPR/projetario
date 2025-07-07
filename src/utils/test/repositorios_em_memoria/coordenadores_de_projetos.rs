use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::vec;

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use futures_util::FutureExt;
use sqlx::{AnyPool, Connection, Executor, FromRow, PgPool, Pool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::{
    Filtro,
    Ordenador,
    Paginacao,
    ProjetosPaginados,
    RepositorioDeCoordenadoresDeProjetos,
};
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
use crate::utils::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;

pub struct ProjetoCoordenadorTupla {
    pub id_professor: Uuid,
    pub id_projeto: Uuid,
}

pub struct RepositorioDeCoordenadoresDeProjetosEmMemoria {
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

    async fn buscar_projetos(
        &self,
        filtro: Filtro,
        ordenador: Ordenador,
        paginacao: Paginacao,
    ) -> Result<ProjetosPaginados, ErroDeDominio> {
        let mut busca =
            QueryBuilder::<Postgres>::new(r#"SELECT id, titulo, tipo, iniciado_em FROM projeto"#);

        match filtro {
            Filtro::Titulo(titulo) => {
                busca
                    .push(" WHERE titulo ILIKE '%' || ")
                    .push_bind(titulo)
                    .push(" || '%'");
            }
            Filtro::TipoProjeto(tipo) => {
                busca.push(" WHERE tipo = ").push_bind(tipo);
            }
        };

        match ordenador {
            Ordenador::Data(ordem) => {
                busca.push(" ORDER BY iniciado_em ");
                match ordem {
                    Ordering::Less => busca.push("ASC"),
                    Ordering::Greater => busca.push("DESC"),
                    Ordering::Equal => busca.push("ASC"),
                };
            }
            Ordenador::Titulo(ordem) => {
                busca.push(" ORDER BY titulo ");
                match ordem {
                    Ordering::Less => busca.push("ASC"),
                    Ordering::Greater => busca.push("DESC"),
                    Ordering::Equal => busca.push("ASC"),
                };
            }
        };

        let limite = (paginacao.pagina - 1) * paginacao.qtd_por_pagina as u32;
        busca
            .push(" LIMIT ")
            .push_bind(paginacao.qtd_por_pagina as i32)
            .push(" OFFSET ")
            .push_bind(limite as i32);

        //let projetos = busca.build_query_as::<Projeto>().fetch_all().await?;

        let projetos = Vec::new();

        Ok(ProjetosPaginados {
            projetos,
            qtd_por_pagina: paginacao.qtd_por_pagina,
        })
    }
}
