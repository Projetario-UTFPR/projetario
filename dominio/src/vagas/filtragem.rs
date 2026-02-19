use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::comum::filtragem::{DirecaoOrdenacao, LimitadorDeData};
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum FiltroDeVaga {
    Titulo(String),
    Tipo(TipoDeProjeto),
    Coordenador(Uuid),
    DataDePublicacao(DbDateTime, LimitadorDeData),
    Estado(EstadoDaVaga),
    // curso
    // campus
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum EstadoDaVaga {
    Ativa,
    Cancelada,
    Encerrada,
}

#[derive(Clone)]
pub enum OrdenacaoDeVaga {
    Data(DirecaoOrdenacao),
    Titulo(DirecaoOrdenacao),
}

impl Default for OrdenacaoDeVaga {
    fn default() -> Self { OrdenacaoDeVaga::Data(Default::default()) }
}
