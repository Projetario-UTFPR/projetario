use sqlx::PgPool;

use crate::dominio::autenticacao::servicos::autenticar_usuario::ServicoAutenticarUsuario;
use crate::dominio::identidade::repositorios::usuarios::RepositorioDeUsuarios;
use crate::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;
use crate::infra::repositorios::sqlx::usuarios::RepositorioDeUsuariosSQLX;

pub fn obtenha_servico_autenticar_usuario(
    db_conn: &PgPool,
) -> ServicoAutenticarUsuario<ComparadorEHasherDeSenhaCrypto, RepositorioDeUsuariosSQLX<'_>> {
    let repositorio_usuarios = RepositorioDeUsuariosSQLX::novo(db_conn);
    ServicoAutenticarUsuario::novo(ComparadorEHasherDeSenhaCrypto::novo(), repositorio_usuarios)
}
