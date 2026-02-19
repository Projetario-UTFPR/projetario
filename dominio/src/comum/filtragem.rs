#[derive(Default, Clone)]
pub enum DirecaoOrdenacao {
    Asc,
    #[default]
    Desc,
}

impl DirecaoOrdenacao {
    pub fn into_sql_string(self) -> String {
        match self {
            DirecaoOrdenacao::Asc => "ASC".to_string(),
            DirecaoOrdenacao::Desc => "DESC".to_string(),
        }
    }
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub enum LimitadorDeData {
    #[default]
    Ate,
    Apos,
}
