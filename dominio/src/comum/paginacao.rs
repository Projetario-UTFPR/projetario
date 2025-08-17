pub const POR_PAGINA_PADRAO: u8 = 12;

/// Esta estrutura serve para trafegar dados de paginação internamente
/// para processar buscas.
///
/// Não se trata de um DTO, tampouco um presenter.
pub struct Paginacao {
    pub pagina: u32,
    pub qtd_por_pagina: u8,
}

impl Default for Paginacao {
    fn default() -> Self { Self::nova(1, POR_PAGINA_PADRAO) }
}

impl Paginacao {
    pub fn nova(pagina: u32, qtd_por_pagina: u8) -> Self {
        Self {
            pagina,
            qtd_por_pagina,
        }
    }

    pub fn nova_por_opcionais(pagina: Option<u32>, qtd_por_pagina: Option<u8>) -> Self {
        Self {
            pagina: pagina.unwrap_or(1),
            qtd_por_pagina: qtd_por_pagina.unwrap_or(POR_PAGINA_PADRAO),
        }
    }

    pub fn com_pagina(mut self, pagina: u32) -> Self {
        self.pagina = pagina;
        self
    }

    pub fn com_qtd_por_pagina(mut self, qtd: u8) -> Self {
        self.qtd_por_pagina = qtd;
        self
    }
}
