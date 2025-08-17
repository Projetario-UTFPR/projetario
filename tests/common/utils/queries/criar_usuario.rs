use dominio::autenticacao::HasherDeSenha;
use dominio::identidade::entidades::usuario::UsuarioModelo;
use projetario::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;
use sqlx::PgPool;

pub async fn salvar_usuario(db_conn: &PgPool, usuario: &UsuarioModelo) {
    let senha = ComparadorEHasherDeSenhaCrypto::novo()
        .aplique_hash(&usuario.senha_hash)
        .unwrap();

    sqlx::query(
        r#"INSERT INTO usuario (
            id, nome, email, senha_hash, url_curriculo_lattes,
            cargo, registrado_em, atualizado_em, desativado_em,
            registro_aluno, periodo)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"#,
    )
    .bind(usuario.id)
    .bind(&usuario.nome)
    .bind(&usuario.email)
    .bind(&senha)
    .bind(&usuario.url_curriculo_lattes)
    .bind(&usuario.cargo)
    .bind(usuario.registrado_em)
    .bind(usuario.atualizado_em)
    .bind(usuario.desativado_em)
    .bind(&usuario.registro_aluno)
    .bind(usuario.periodo.to_optional::<i16>())
    .execute(db_conn)
    .await
    .expect("Não foi possível inserir o usuário no banco de dados para os testes");
}
