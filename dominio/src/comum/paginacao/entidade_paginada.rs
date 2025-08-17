use std::fmt::Debug;

#[derive(Debug)]
pub struct EntidadePaginada<Entidade: Debug> {
    pub dados: Vec<Entidade>,
    pub qtd_total: u64,
}
