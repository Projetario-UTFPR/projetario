use serde::Deserialize;
use uuid::Uuid;

use crate::comum::filtragem::DirecaoOrdenacao;

#[derive(Deserialize)]
pub enum OrdenacaoDeProjeto {
    /// Ordenar os projetos pela sua data na ordem fornecida.
    Data(DirecaoOrdenacao),
    /// Ordenar os projetos pelo título alfabeticamente na ordem fornecida.
    Titulo(DirecaoOrdenacao),
}

impl Default for OrdenacaoDeProjeto {
    fn default() -> Self { OrdenacaoDeProjeto::Data(Default::default()) }
}

#[derive(serde::Deserialize)]
pub enum FiltroDeProjeto {
    /// Filtrar projetos que incluem o título
    Titulo(String),
}
