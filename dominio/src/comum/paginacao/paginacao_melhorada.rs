use std::fmt::Debug;

use crate::comum::paginacao::EntidadePaginada;

pub struct PaginacaoMelhorada<Entidade: Debug> {
    pub total: u64,
    pub pagina_atual: u64,
    pub ultima_pagina: u64,
    pub primeira_pagina: u8,
    pub por_pagina: u8,
    pub dados: Vec<Entidade>,
}

impl<Entidade: Debug> PaginacaoMelhorada<Entidade> {
    pub fn nova(dados: Vec<Entidade>, total: u64, por_pagina: u8, pagina_atual: u64) -> Self {
        let ultima_pagina = total.div_ceil(por_pagina as u64);

        Self {
            dados,
            pagina_atual,
            por_pagina,
            primeira_pagina: 1,
            total,
            ultima_pagina,
        }
    }

    pub fn nova_a_partir_de_entidade_paginada(
        paginacao: EntidadePaginada<Entidade>,
        por_pagina: u8,
        pagina_atual: u64,
    ) -> Self {
        Self::nova(
            paginacao.dados,
            paginacao.qtd_total,
            por_pagina,
            pagina_atual,
        )
    }
}
