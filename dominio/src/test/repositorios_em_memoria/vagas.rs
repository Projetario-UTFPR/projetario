use std::cmp::Reverse;
use std::collections::HashSet;

use async_trait::async_trait;
use comum::erros::{ErroDeDominio, ResultadoDominio};
use uuid::Uuid;

use crate::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use crate::comum::filtragem::{DirecaoOrdenacao, LimitadorDeData};
use crate::comum::paginacao::{EntidadePaginada, Paginacao};
use crate::test::repositorios_em_memoria::TabelaThreadSafeEmMemoria;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::filtragem::{EstadoDaVaga, FiltroDeVaga, OrdenacaoDeVaga};
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

    async fn buscar_vagas(
        &self,
        filtros: HashSet<FiltroDeVaga>,
        ordenador: OrdenacaoDeVaga,
        paginacao: Paginacao,
    ) -> ResultadoDominio<EntidadePaginada<Vaga>> {
        let mut vagas = self.vagas_tbl.lock().unwrap().to_vec();

        for filtro in filtros {
            vagas = Self::filtre_vagas(vagas, filtro);
        }

        Self::ordene_vagas(&mut vagas, ordenador);

        let contagem = vagas.len();

        let vagas = vagas
            .into_iter()
            .skip(paginacao.calcule_offset() as usize)
            .take(paginacao.qtd_por_pagina as usize)
            .collect();

        Ok(EntidadePaginada {
            dados: vagas,
            qtd_total: contagem as u64,
        })
    }
}

impl RepositorioDeVagasEmMemoria {
    fn filtre_vagas(vagas: Vec<Vaga>, filtro: FiltroDeVaga) -> Vec<Vaga> {
        vagas
            .into_iter()
            .filter(|vaga| match &filtro {
                FiltroDeVaga::Coordenador(id) => {
                    vaga.obtenha_coordenador()
                        .obtenha_usuario()
                        .obtenha_id()
                        .eq(id)
                        || vaga
                            .obtenha_vice_coordenador()
                            .is_some_and(|vice| vice.obtenha_usuario().obtenha_id().eq(id))
                }
                FiltroDeVaga::DataDePublicacao(data, limitador) => match limitador {
                    LimitadorDeData::Apos => vaga.obtenha_data_de_inicio() > data.date(),
                    LimitadorDeData::Ate => vaga.obtenha_data_de_inicio() <= data.date(),
                },
                FiltroDeVaga::Estado(estado) => match estado {
                    EstadoDaVaga::Ativa => vaga.esta_ativa(),
                    EstadoDaVaga::Cancelada => vaga.obtenha_data_de_cancelamento().is_some(),
                    EstadoDaVaga::Encerrada => {
                        !vaga.esta_ativa() && vaga.obtenha_data_de_cancelamento().is_none()
                    }
                },
                FiltroDeVaga::Tipo(tipo) => vaga.obtenha_projeto().obtenha_tipo().eq(tipo),
                FiltroDeVaga::Titulo(titulo) => {
                    vaga.obtenha_titulo().to_lowercase().contains(titulo)
                }
            })
            .collect()
    }

    fn ordene_vagas(vagas: &mut [Vaga], ordenador: OrdenacaoDeVaga) {
        match ordenador {
            OrdenacaoDeVaga::Data(direcao) => match direcao {
                DirecaoOrdenacao::Asc => {
                    vagas.sort_by_key(|vaga| vaga.obtenha_data_de_inicio());
                }
                DirecaoOrdenacao::Desc => {
                    vagas.sort_by_key(|vaga| Reverse(vaga.obtenha_data_de_inicio()));
                }
            },
            OrdenacaoDeVaga::Titulo(direcao) => match direcao {
                DirecaoOrdenacao::Asc => {
                    vagas.sort_by_key(|vaga| vaga.obtenha_titulo().to_owned());
                }
                DirecaoOrdenacao::Desc => {
                    vagas.sort_by_key(|vaga| Reverse(vaga.obtenha_titulo().to_owned()));
                }
            },
        };
    }
}
