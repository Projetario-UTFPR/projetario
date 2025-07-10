/*
FIXME: Descomentar e ajustar quando estiver implementada em sqlx
use sqlx::PgPool;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::dominio::vagas::servicos::criar_vaga::ServicoCriarVaga;

pub fn obtenha_servico_criar_vaga(pool: &PgPool) -> ServicoCriarVaga<RepositorioDeVagasSQLX> {
    let repo = RepositorioDeVagas::novo(pool);
    ServicoCriarVaga::novo(repo)
}
*/
