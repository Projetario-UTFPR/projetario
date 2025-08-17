use std::fmt::Debug;

use dominio::comum::paginacao::PaginacaoMelhorada;
use serde::Serialize;

#[derive(Serialize)]
pub struct PaginacaoMelhoradaPresenter<EntidadePaginada> {
    pub total: u64,
    #[serde(rename = "paginaAtual")]
    pub pagina_atual: u64,
    #[serde(rename = "ultimaPagina")]
    pub ultima_pagina: u64,
    #[serde(rename = "primeiraPagina")]
    pub primeira_pagina: u8,
    #[serde(rename = "porPagina")]
    pub por_pagina: u8,
    pub dados: Vec<EntidadePaginada>,
}

impl<EntidadePaginada> PaginacaoMelhoradaPresenter<EntidadePaginada> {
    pub fn apresente<Entidade: Debug>(
        paginacao: &PaginacaoMelhorada<Entidade>,
        presenter: fn(&Entidade) -> EntidadePaginada,
    ) -> Self {
        Self {
            dados: paginacao.dados.iter().map(presenter).collect::<Vec<_>>(),
            pagina_atual: paginacao.pagina_atual,
            por_pagina: paginacao.por_pagina,
            primeira_pagina: paginacao.primeira_pagina,
            total: paginacao.total,
            ultima_pagina: paginacao.ultima_pagina,
        }
    }
}
