use async_trait::async_trait;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
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
        ordenadador: Ordenador,
        paginacao: Paginacao,
    ) -> ProjetosPaginados {
        let mut select_query =
            QueryBuilder::<Postgres>::new(r#"SELECT id, titulo, tipo, iniciado_em FROM projetos"#);

        match filtro {
            Filtro::Titulo(titulo) => {
                select_query
                    .push(" WHERE titulo ILIKE '%' || ")
                    .push_bind(titulo)
                    .push(" || '%'");
            }
            Filtro::TipoProjeto(tipo) => {
                select_query.push(" WHERE tipo = ").push_bind(tipo);
            }
        }

        match ordenador {
            Ordenador::Data(ordem) => {
                select_query.push(" ORDER BY iniciado_em ");
                match ordem {
                    Ordering::Asc => select_query.push("ASC"),
                    Ordering::Desc => select_query.push("DESC"),
                }
            }
            Ordenador::Titulo(ordem) => {
                select_query.push(" ORDER BY titulo ");
                match ordem {
                    Ordering::Asc => select_query.push("ASC"),
                    Ordering::Desc => select_query.push("DESC"),
                }
            }
        }

        let offset = (paginacao.pagina - 1) * paginacao.qtd_por_pagina as u32;
        select_query
            .push(" LIMIT ")
            .push_bind(paginacao.qtd_por_pagina as i32)
            .push(" OFFSET ")
            .push_bind(offset as i32);

        let projetos = select_query
            .build_query_as()
            .fetch_all(self.datastore.get_db())
            .await?;

        Ok(ProjetosPaginados {
            projetos,
            qtd_por_pagina: paginacao.qtd_por_pagina,
        })
    }
}
