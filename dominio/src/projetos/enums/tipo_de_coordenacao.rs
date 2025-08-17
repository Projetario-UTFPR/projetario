use std::fmt::Display;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "tipo_coordenacao_e", rename_all = "snake_case")]
pub enum TipoDeCoordenacao {
    Coordenador,
    ViceCoordenador,
}

impl Display for TipoDeCoordenacao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Coordenador => "coordenador",
                Self::ViceCoordenador => "vice-coordenador",
            }
        )
    }
}
