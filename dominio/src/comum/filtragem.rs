use serde::Deserialize;

#[derive(Deserialize, Default)]
pub enum DirecaoOrdenacao {
    Asc,
    #[default]
    Desc,
}
