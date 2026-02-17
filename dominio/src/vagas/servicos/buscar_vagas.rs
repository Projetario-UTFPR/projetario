use std::collections::HashSet;

use comum::erros::ResultadoDominio;
use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::comum::filtragem::LimitadorDeData;
use crate::comum::paginacao::{Paginacao, PaginacaoMelhorada};
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::filtragem::{EstadoDaVaga, FiltroDeVaga, OrdenacaoDeVaga};
use crate::vagas::repositorios::vaga::RepositorioDeVagas;

#[derive(Default)]
#[cfg_attr(dev_utils, derive(derive_builder::Builder, Clone))]
#[cfg_attr(dev_utils, builder(setter(into), default))]
pub struct PossiveisFiltrosParaBuscarVagas {
    pub titulo: Option<String>,
    pub tipo: Option<TipoDeProjeto>,
    pub coordenador: Option<Uuid>,
    pub data_de_publicacao: Option<(DbDateTime, LimitadorDeData)>,
    pub estado: Option<EstadoDaVaga>,
}

pub struct BuscarVagasDeProjetosParams {
    pub possiveis_filtros: PossiveisFiltrosParaBuscarVagas,
    pub ordenador: Option<OrdenacaoDeVaga>,
    pub paginacao: Option<Paginacao>,
}

pub struct ServicoBuscarVagasDeProjetos<RV: RepositorioDeVagas> {
    repositorio_de_vagas: RV,
}

impl<RV: RepositorioDeVagas> ServicoBuscarVagasDeProjetos<RV> {
    pub fn novo(repositorio_de_vagas: RV) -> Self {
        Self {
            repositorio_de_vagas,
        }
    }

    pub async fn executar(
        &self,
        BuscarVagasDeProjetosParams {
            possiveis_filtros,
            ordenador,
            paginacao,
        }: BuscarVagasDeProjetosParams,
    ) -> ResultadoDominio<PaginacaoMelhorada<Vaga>> {
        let filtros = Self::transforme_filtros_em_hashset(possiveis_filtros);

        let paginacao = paginacao.unwrap_or_default();

        let vagas = self
            .repositorio_de_vagas
            .buscar_vagas(filtros, ordenador.unwrap_or_default(), paginacao.clone())
            .await?;

        Ok(PaginacaoMelhorada::nova_a_partir_de_entidade_paginada(
            vagas,
            paginacao.qtd_por_pagina,
            paginacao.pagina,
        ))
    }

    fn transforme_filtros_em_hashset(
        params: PossiveisFiltrosParaBuscarVagas,
    ) -> HashSet<FiltroDeVaga> {
        let mut filtros = HashSet::new();

        if let Some(value) = params.coordenador {
            filtros.insert(FiltroDeVaga::Coordenador(value));
        }

        if let Some(value) = params.data_de_publicacao {
            filtros.insert(FiltroDeVaga::DataDePublicacao(value.0, value.1));
        }

        if let Some(value) = params.estado {
            filtros.insert(FiltroDeVaga::Estado(value));
        }

        if let Some(value) = params.titulo {
            filtros.insert(FiltroDeVaga::Titulo(value));
        }

        if let Some(value) = params.tipo {
            filtros.insert(FiltroDeVaga::Tipo(value));
        }

        filtros
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;

    use super::*;
    use crate::test::arbitrary::*;
    use crate::test::repositorios_em_memoria::vagas::RepositorioDeVagasEmMemoria;

    type Sut = ServicoBuscarVagasDeProjetos<RepositorioDeVagasEmMemoria>;

    fn filtros_batem(
        filtro_parseado: &FiltroDeVaga,
        filtros: &PossiveisFiltrosParaBuscarVagas,
    ) -> bool {
        match filtro_parseado {
            FiltroDeVaga::Titulo(titulo) => filtros
                .titulo
                .as_ref()
                .is_some_and(|_titulo| _titulo.eq(titulo)),
            FiltroDeVaga::Tipo(tipo_de_projeto) => {
                filtros.tipo.is_some_and(|tipo| tipo.eq(tipo_de_projeto))
            }
            FiltroDeVaga::Coordenador(uuid) => filtros.coordenador.is_some_and(|id| id.eq(uuid)),
            FiltroDeVaga::DataDePublicacao(data, limitador) => filtros
                .data_de_publicacao
                .as_ref()
                .is_some_and(|(_data, _limitador)| _data.eq(data) && _limitador.eq(limitador)),
            FiltroDeVaga::Estado(estado) => filtros
                .estado
                .as_ref()
                .is_some_and(|_estado| _estado.eq(estado)),
        }
    }

    proptest! {
        #[test]
        fn transforma_filtros_corretamente(
            titulo in proptest::option::of(".*"),
            tipo in proptest::option::of(arb_tipo()),
            coordenador in proptest::option::of(arb_uuid()),
            data in proptest::option::of((arb_db_date_time(), arb_limitador())),
            estado in proptest::option::of(arb_estado()),
        ) {
            let filtros = PossiveisFiltrosParaBuscarVagas {
                titulo,
                tipo,
                coordenador,
                data_de_publicacao: data,
                estado,
            };

            let resultado = Sut::transforme_filtros_em_hashset(filtros.clone());

            let esperado = [
                filtros.titulo.is_some(),
                filtros.tipo.is_some(),
                filtros.coordenador.is_some(),
                filtros.data_de_publicacao.is_some(),
                filtros.estado.is_some(),
            ]
            .into_iter()
            .filter(|b| *b)
            .count();

            prop_assert_eq!(resultado.len(), esperado);

            for filtro in resultado {
                prop_assert!(filtros_batem(&filtro, &filtros));
            }
        }
    }
}
