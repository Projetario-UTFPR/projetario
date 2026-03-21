use dominio::vagas::servicos::buscar_vagas::ServicoBuscarVagasDeProjetos;
use sqlx::PgPool;

use crate::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;

pub fn obtenha_servico_buscar_vagas(
    db_conn: &'_ PgPool,
) -> ServicoBuscarVagasDeProjetos<RepositorioDeVagasSQLX<'_>> {
    let repositorio_de_vagas = RepositorioDeVagasSQLX::novo(db_conn);
    ServicoBuscarVagasDeProjetos::novo(repositorio_de_vagas)
}
