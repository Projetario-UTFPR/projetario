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

pub struct BuscarProjetosParams {
    pub filtro: Filtro,
    pub tipo: Option<Tipo>,
    pub ordenador: Ordenador,
    pub pagina: u32,
}

pub struct ServicoBuscaProjetos<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_coordenadores: RCP,
}
impl<RCP> ServicoBuscaProjetos<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_coordenadores: RCP) -> Self {
        Self {
            repositorio_de_coordenadores,
        }
    }

    pub async fn buscar_projeto(
        &self,
        params: BuscarProjetosParams,
    ) -> Result<ProjetosPaginados, ErroDeDominio> {
        let BuscarProjetosParams {
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

        Ok(self
            .repositorio_de_coordenadores
            .buscar_projetos(filtro, tipo, ordenador, paginacao)
            .await?)
    }
}
