use serde::Serialize;

/// Esse tipo de estrutura deve ter seu erro tratado no client-side, diferente de
/// outras abordagens onde erros param a renderização da página imediatamente,
/// ocasionando em uma página de erro por inteiro.
#[derive(Serialize)]
pub struct RespostaIncertaDoServidor<T: Serialize> {
    pub success: bool,
    pub error: Option<String>,
    pub data: Option<T>,
}
