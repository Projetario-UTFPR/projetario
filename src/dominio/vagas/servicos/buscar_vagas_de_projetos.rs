use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::{
    Filtro,
    Ordenador,
    Paginacao,
    ProjetosPaginados,
    RepositorioDeCoordenadoresDeProjetos,
    Tipo,
};
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub struct BuscarVagasDeProjetosParams {
    pub filtro: Filtro,
    pub tipo: Option<Tipo>,
    pub ordenador: Ordenador,
    pub pagina: u32,
}

pub struct ServicoBuscarVagasDeProjetos<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_coordenadores: RCP,
}
impl<RCP> ServicoBuscarVagasDeProjetos<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_coordenadores: RCP) -> Self {
        Self {
            repositorio_de_coordenadores,
        }
    }

    pub async fn executar(
        &self,
        params: BuscarVagasDeProjetosParams,
    ) -> Result<ProjetosPaginados, ErroDeDominio> {
        let BuscarVagasDeProjetosParams {
            filtro,
            tipo,
            ordenador,
            pagina,
        } = params;

        let qtd_por_pagina: u8 = 10;

        let paginacao = Paginacao {
            pagina,
            qtd_por_pagina,
        };

        self.repositorio_de_coordenadores
            .buscar_projetos(filtro, tipo, ordenador, paginacao)
            .await
    }
}
