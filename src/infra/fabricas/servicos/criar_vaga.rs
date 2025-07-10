use sqlx::PgPool;

use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::dominio::vagas::servicos::criar_vaga::ServicoCriarVaga;
use crate::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;

pub fn obtenha_servico_criar_vaga(pool: &PgPool) -> ServicoCriarVaga<RepositorioDeVagasSQLX<'_>> {
    let repo = RepositorioDeVagasSQLX::novo(pool);
    ServicoCriarVaga::novo(repo)
}
