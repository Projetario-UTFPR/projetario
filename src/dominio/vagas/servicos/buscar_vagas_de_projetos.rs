use crate::comum::paginacao::{POR_PAGINA_PADRAO, Paginacao};
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::filtragem::{FiltroDeProjeto, OrdenacaoDeProjeto};
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::{
    ProjetosPaginados,
    RepositorioDeCoordenadoresDeProjetos,
};
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub struct BuscarVagasDeProjetosParams {
    pub filtro: Option<FiltroDeProjeto>,
    pub tipo: Option<TipoDeProjeto>,
    pub ordenador: Option<OrdenacaoDeProjeto>,
    pub paginacao: Paginacao,
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
            paginacao,
        } = params;

        self.repositorio_de_coordenadores
            .buscar_projetos(filtro, tipo, ordenador.unwrap_or_default(), paginacao)
            .await
    }
}
