#[derive(Debug)]
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
pub enum Cargo {
    Aluno,
    Professor,
    Administrador,
}
