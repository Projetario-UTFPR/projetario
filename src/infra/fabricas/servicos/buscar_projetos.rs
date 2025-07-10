use sqlx::PgPool;

use crate::dominio::vagas::servicos::buscar_vagas_de_projetos::ServicoBuscarVagasDeProjetos;
use crate::infra::repositorios::sqlx::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosSQLX;

pub fn obtenha_servico_buscar_projetos(
    db_conn: &PgPool,
) -> ServicoBuscarVagasDeProjetos<RepositorioDeCoordenadoresDeProjetosSQLX> {
    let repositorio_coordenadores = RepositorioDeCoordenadoresDeProjetosSQLX::novo(db_conn);
    ServicoBuscarVagasDeProjetos::novo(repositorio_coordenadores)
}
