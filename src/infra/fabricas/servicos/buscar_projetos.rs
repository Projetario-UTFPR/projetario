use sqlx::PgPool;

use crate::dominio::projetos::servicos::buscar_projetos::ServicoBuscaProjetos;
use crate::infra::repositorios::sqlx::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosSQLX;

pub fn obtenha_servico_buscar_projetos(
    db_conn: &PgPool,
) -> ServicoBuscaProjetos<RepositorioDeCoordenadoresDeProjetosSQLX> {
    let repositorio_coordenadores = RepositorioDeCoordenadoresDeProjetosSQLX::novo(db_conn);
    ServicoBuscaProjetos::novo(repositorio_coordenadores)
}
