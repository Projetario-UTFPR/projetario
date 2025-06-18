use sqlx::PgPool;

use crate::dominio::projetos::servicos::criar_projeto_de_extensao::ServicoCriarProjetoDeExtensao;
use crate::infra::repositorios::sqlx::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosSQLX;

pub fn obtenha_servico_criar_projeto_de_extensao(
    db_conn: &PgPool,
) -> ServicoCriarProjetoDeExtensao<RepositorioDeCoordenadoresDeProjetosSQLX> {
    let repositorio_coordenadores = RepositorioDeCoordenadoresDeProjetosSQLX::novo(db_conn);
    ServicoCriarProjetoDeExtensao::novo(repositorio_coordenadores)
}
