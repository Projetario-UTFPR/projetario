#[derive(Debug, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "tipo_de_projeto_e", rename_all = "snake_case")]
pub enum TipoDeProjeto {
    Extensao,
    IniciacaoCientifica,
}
