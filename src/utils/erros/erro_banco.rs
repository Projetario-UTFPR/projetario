use sqlx::Error as SqlxError;

#[derive(Debug)]
pub struct BancoDeDadosError(pub SqlxError);

impl std::fmt::Display for BancoDeDadosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Erro de banco de dados: {}", self.0)
    }
}

impl std::error::Error for BancoDeDadosError {}

impl From<SqlxError> for BancoDeDadosError {
    fn from(e: SqlxError) -> Self {
        BancoDeDadosError(e)
    }
}