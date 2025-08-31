use comum::erros::ResultadoDominio;

use crate::comum::paginacao::{Paginacao, PaginacaoMelhorada};
use crate::projetos::entidades::projeto::Projeto;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::projetos::filtragem::{EstadoDoProjeto, FiltroDeProjeto, OrdenacaoDeProjeto};
use crate::projetos::repositorios::projetos::RepositorioDeProjetos;

pub struct BuscarProjetosDeExtensaoParams {
    pub filtro: Option<FiltroDeProjeto>,
    pub estado: Option<EstadoDoProjeto>,
    pub ordenador: Option<OrdenacaoDeProjeto>,
    pub paginacao: Paginacao,
}

pub struct ServicoBuscarProjetoDeExtensao<RP: RepositorioDeProjetos> {
    repositorio_de_projetos: RP,
}

impl<RP: RepositorioDeProjetos> ServicoBuscarProjetoDeExtensao<RP> {
    pub fn novo(repositorio_de_projetos: RP) -> Self {
        Self {
            repositorio_de_projetos,
        }
    }

    pub async fn executar(
        &self,
        params: BuscarProjetosDeExtensaoParams,
    ) -> ResultadoDominio<PaginacaoMelhorada<Projeto>> {
        let projetos = self
            .repositorio_de_projetos
            .buscar_projetos(
                params.filtro,
                params.estado,
                Some(TipoDeProjeto::Extensao),
                params.ordenador.unwrap_or_default(),
                params.paginacao.clone(),
            )
            .await?;

        Ok(PaginacaoMelhorada::nova_a_partir_de_entidade_paginada(
            projetos,
            params.paginacao.qtd_por_pagina,
            params.paginacao.pagina,
        ))
    }
}

// TODO: implementar testes unitários para o serviço de buscar projetos de extensao
