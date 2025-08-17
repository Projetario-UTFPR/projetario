pub struct EntidadePaginada<Entidade> {
    pub dados: Vec<Entidade>,
    pub qtd_total: u64,
}
