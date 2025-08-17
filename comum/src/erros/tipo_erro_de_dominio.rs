#[derive(Debug, Clone)]
pub enum TipoErroDeDominio {
    Interno,
    Integridade,
    ValorInvalido,
    NaoAutorizado,
    NaoEncontrado,
    RegraDeNegocio,
}

impl TipoErroDeDominio {
    pub fn como_codigo(&self) -> String {
        match self {
            Self::Integridade => "integridade",
            Self::Interno => "interno",
            Self::NaoAutorizado => "não_autorizado",
            Self::ValorInvalido => "valor_inválido",
            Self::NaoEncontrado => "nao_encontrado",
            Self::RegraDeNegocio => "regra_de_negocio",
        }
        .into()
    }
}
