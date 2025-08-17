use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// O cargo da pessoa como um usuário na aplicação.
///
/// ### Aluno
/// Representa um aluno comum.
///
/// ### Professor
/// Representa um usuário professor comum.
///
/// ### Administrador
/// Um professor com permissão para manipular projetos de terceiros e para gerenciar tudo o
/// que for gerenciável na plataforma.
///
/// É destinado a coordenadores de cursos e outros funcionários que devam ter
/// esse nível de permissão dentro da plataforma.
#[derive(Debug, PartialEq, Eq, Clone, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "cargo_e", rename_all = "snake_case")]
pub enum Cargo {
    Aluno,
    Professor,
    Administrador,
}

impl Display for Cargo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{self:#?}") }
}
