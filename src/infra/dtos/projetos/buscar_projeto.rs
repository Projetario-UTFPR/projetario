use serde::Deserialize;
use validator::Validate;

use crate::dominio::projetos::repositorios::coordenadores_de_projetos::{Filtro, Ordenador, Tipo};

#[derive(Deserialize, Validate)]
pub struct BuscarProjetoDto {
    pub filtro: Filtro,
    pub tipo: Option<Tipo>,
    pub ordenador: Ordenador,
    pub pagina_atual: u32,
}
