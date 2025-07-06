#[derive(Debug, Clone)]
pub enum TipoErroDeDominio {
    Interno,
    Integridade,
    ValorInvalido,
    NãoAutorizado,
}

impl TipoErroDeDominio {
    pub fn como_codigo(&self) -> String {
        match self {
            Self::Integridade => "integridade",
            Self::Interno => "interno",
            Self::NãoAutorizado => "não_autorizado",
            Self::ValorInvalido => "valor_inválido",
        }
        .into()
    }
}
