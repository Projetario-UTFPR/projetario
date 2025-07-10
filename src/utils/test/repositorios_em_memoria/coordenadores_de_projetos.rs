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
    DirecaoOrdenacao,
    Filtro,
    Ordenador,
    Paginacao,
    ProjetosPaginados,
    RepositorioDeCoordenadoresDeProjetos,
    Tipo,
};
use crate::utils::erros::ResultadoDominio;
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
        tipo: Option<Tipo>,
        ordenador: Ordenador,
        paginacao: Paginacao,
    ) -> Result<ProjetosPaginados, ErroDeDominio> {
        let mut busca = QueryBuilder::<Postgres>::new(
            r#"SELECT id, titulo, descricao, tipo, registrado_em, iniciado_em, atualizado_em, cancelado_em, concluido_em FROM projeto"#,
        );

        let mut tem_condicoes = false;

        match filtro {
            Filtro::Titulo(titulo) => {
                busca.push(" WHERE titulo ILIKE '%' || ");
                busca.push_bind(titulo);
                busca.push(" || '%'");
                tem_condicoes = true;
            }
        }

        if let Some(Tipo::Tipo(tipo)) = tipo {
            if tem_condicoes {
                busca.push(" AND tipo = ");
            } else {
                busca.push(" WHERE tipo = ");
                tem_condicoes = true;
            }
            busca.push_bind(tipo);
        }

        match ordenador {
            Ordenador::Data(ordem) => {
                busca.push(" ORDER BY iniciado_em ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
                };
            }
            Ordenador::Titulo(ordem) => {
                busca.push(" ORDER BY titulo ");
                match ordem {
                    DirecaoOrdenacao::Asc => busca.push("ASC"),
                    DirecaoOrdenacao::Desc => busca.push("DESC"),
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

    async fn buscar_coordenadores_do_projeto(
        &self,
        projeto: &Projeto,
    ) -> ResultadoDominio<(Professor, Option<Professor>)> {
        todo!()
    }
}
