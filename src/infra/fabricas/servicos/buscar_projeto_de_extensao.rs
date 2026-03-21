use dominio::projetos::servicos::buscar_projetos_de_extensao::ServicoBuscarProjetoDeExtensao;
use sqlx::PgPool;

use crate::infra::repositorios::sqlx::projetos::RepositorioDeProjetosSQLX;

pub fn obtenha_servico_buscar_de_projetos_de_extensao(
    db_conn: &'_ PgPool,
) -> ServicoBuscarProjetoDeExtensao<RepositorioDeProjetosSQLX<'_>> {
    let repositorio_projetos = RepositorioDeProjetosSQLX::novo(db_conn);
    ServicoBuscarProjetoDeExtensao::novo(repositorio_projetos)
}
