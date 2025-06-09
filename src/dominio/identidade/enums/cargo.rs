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
#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "cargo_e", rename_all = "snake_case")]
pub enum Cargo {
    Aluno,
    Professor,
    Administrador,
}
