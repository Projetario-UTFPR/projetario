use dominio::autenticacao::HasherDeSenha;
use dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use dominio::identidade::enums::cargo::Cargo;
use projetario::infra::crypto::comparador_e_hasher_de_senhas::ComparadorEHasherDeSenhaCrypto;
use sqlx::{PgPool, Postgres};

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

pub async fn salvar_usuarios(db_conn: &PgPool, usuarios: Vec<&Usuario>, fazer_hash_da_senha: bool) {
    let mut query = sqlx::QueryBuilder::<Postgres>::new(
        r#"INSERT INTO usuario (
            id, nome, email, senha_hash, url_curriculo_lattes,
            cargo, registrado_em, atualizado_em, desativado_em
        ) "#,
    );

    let usuarios = if fazer_hash_da_senha {
        let hasher = ComparadorEHasherDeSenhaCrypto::novo();
        usuarios
            .into_iter()
            .cloned()
            .map(|mut usuario| {
                let senha = hasher
                    .aplique_hash(
                        usuario
                            .obtenha_hash_da_senha()
                            .expect("Tentou salvar um usuário não ativado"),
                    )
                    .unwrap();

                let _ = usuario.coloque_senha(senha);
                usuario
            })
            .collect::<Vec<_>>()
    } else {
        usuarios.into_iter().cloned().collect()
    };

    query.push_values(usuarios.iter().collect::<Vec<_>>(), |mut b, usuario| {
        b.push_bind(usuario.obtenha_id())
            .push_bind(usuario.obtenha_nome())
            .push_bind(usuario.obtenha_email())
            .push_bind(usuario.obtenha_hash_da_senha())
            .push_bind(usuario.obtenha_url_do_curriculo_lattes())
            .push_bind(Cargo::Professor)
            .push_bind(usuario.obtenha_data_de_registro())
            .push_bind(usuario.obtenha_data_de_modificacao())
            .push_bind(usuario.obtenha_data_de_desativacao());
    });

    query
        .build()
        .execute(db_conn)
        .await
        .expect("Falhou ao persistir vários usuários no banco de dados.");
}
