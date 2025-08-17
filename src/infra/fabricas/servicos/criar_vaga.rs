use dominio::vagas::servicos::criar_vaga::ServicoCriarVaga;
use sqlx::PgPool;

use crate::infra::repositorios::sqlx::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetosSQLX;
use crate::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;

pub fn obtenha_servico_criar_vaga(
    pool: &PgPool,
) -> ServicoCriarVaga<RepositorioDeVagasSQLX<'_>, RepositorioDeCoordenadoresDeProjetosSQLX<'_>> {
    let repositorio_de_vagas = RepositorioDeVagasSQLX::novo(pool);
    let repositorio_de_coordenadores = RepositorioDeCoordenadoresDeProjetosSQLX::novo(pool);
    ServicoCriarVaga::novo(repositorio_de_vagas, repositorio_de_coordenadores)
}
