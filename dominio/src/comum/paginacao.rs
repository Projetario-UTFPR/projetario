mod entidade_paginada;
mod paginacao_melhorada;

pub use entidade_paginada::*;
pub use paginacao_melhorada::*;

pub const POR_PAGINA_PADRAO: u8 = 12;

/// Esta estrutura serve para trafegar dados de paginação internamente
/// para processar buscas.
///
/// Não se trata de um DTO, tampouco um presenter.
#[derive(Clone)]
pub struct Paginacao {
    pub pagina: u64,
    pub qtd_por_pagina: u8,
}

impl Default for Paginacao {
    fn default() -> Self { Self::nova(1, POR_PAGINA_PADRAO) }
}

impl Paginacao {
    pub fn nova(pagina: u64, qtd_por_pagina: u8) -> Self {
        Self {
            pagina,
            qtd_por_pagina,
        }
    }

    pub fn nova_por_opcionais(pagina: Option<u64>, qtd_por_pagina: Option<u8>) -> Self {
        Self {
            pagina: pagina.unwrap_or(1),
            qtd_por_pagina: qtd_por_pagina.unwrap_or(POR_PAGINA_PADRAO),
        }
    }

    pub fn com_pagina(mut self, pagina: u64) -> Self {
        self.pagina = pagina;
        self
    }

    pub fn com_qtd_por_pagina(mut self, qtd: u8) -> Self {
        self.qtd_por_pagina = qtd;
        self
    }

    pub fn calcule_offset(&self) -> u64 { (self.pagina - 1) * self.qtd_por_pagina as u64 }
}
