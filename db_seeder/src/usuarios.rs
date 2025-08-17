use dominio::identidade::entidades::aluno::Aluno;
use dominio::identidade::entidades::professor::Professor;
use dominio::identidade::entidades::usuario::UsuarioModelo;
use dominio::identidade::enums::cargo::Cargo;
use dominio::test::fabricas_de_entidades::usuario_modelo::UsuarioModeloParcial;
use sqlx::PgPool;

use crate::senhas::aplicar_hash;

#[allow(dead_code)]
pub struct UsuariosCriados {
    pub admin: Professor,
    pub professor: Professor,
    pub aluno: Aluno,
}

pub async fn inserir_usuarios(db_pool: &PgPool) -> UsuariosCriados {
    let administrador = salvar(
        db_pool,
        UsuarioModeloParcial {
            nome: Some("Paulo Sabo".into()),
            email: Some("sabo@utfpr.com".into()),
            senha_hash: Some("12345".into()),
            cargo: Some(Cargo::Administrador),
            ..Default::default()
        },
    );

    let professor = salvar(
        db_pool,
        UsuarioModeloParcial {
            nome: Some("Reginaldo Ré".into()),
            email: Some("reginaldo@utfpr.com".into()),
            senha_hash: Some("12345".into()),
            cargo: Some(Cargo::Professor),
            ..Default::default()
        },
    );

    let aluno = salvar(
        db_pool,
        UsuarioModeloParcial {
            nome: Some("Pedro Alberto".into()),
            email: Some("pedroalberto@alunos.utfpr.com".into()),
            senha_hash: Some("12345".into()),
            cargo: Some(Cargo::Aluno),
            registro_aluno: Some("a2250331".into()),
            ..Default::default()
        },
    );

    let (prof, admin, aluno): (UsuarioModelo, UsuarioModelo, UsuarioModelo) =
        tokio::join!(professor, administrador, aluno);

    UsuariosCriados {
        admin: Professor::try_from(&admin).unwrap(),
        professor: Professor::try_from(&prof).unwrap(),
        aluno: Aluno::try_from(&aluno).unwrap(),
    }
}

async fn salvar(db_pool: &PgPool, usuario: UsuarioModeloParcial) -> UsuarioModelo {
    let usuario = usuario.into_entidade();

    sqlx::query(concat!(
        "INSERT INTO usuario ",
        "(nome, email, senha_hash, cargo, registro_aluno, periodo) ",
        "SELECT $1, $2, $3, $4, $5, $6 ",
        "WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE email = $2 )",
    ))
    .bind(usuario.nome.clone())
    .bind(usuario.email.clone())
    .bind(aplicar_hash(usuario.senha_hash.as_str()))
    .bind(usuario.cargo.clone())
    .bind(usuario.registro_aluno.clone())
    .bind(usuario.periodo.to_optional::<i32>())
    .execute(db_pool)
    .await
    .expect("Não foi possível inserir o usuário no banco de dados");

    let usuario_mais_atual: UsuarioModelo =
        sqlx::query_as("SELECT * FROM usuario WHERE email = $1")
            .bind(usuario.email.clone())
            .fetch_one(db_pool)
            .await
            .expect("O usuário não foi inserido corretamente");

    match usuario.cargo {
        Cargo::Aluno => log::info!(
            "Adicionado o aluno {} com as credenciais: {}, {}, {}",
            usuario.nome,
            usuario.email,
            usuario.senha_hash,
            usuario.registro_aluno.as_ref().unwrap()
        ),
        _ => log::info!(
            "Adicionado o {} {} com as credenciais: {}, {}",
            usuario.cargo,
            usuario.nome,
            usuario.email,
            usuario.senha_hash
        ),
    }

    usuario_mais_atual
}

// id                  UUID            NOT NULL    DEFAULT gen_random_uuid(),
// -- tamanho máximo sugerido pelo governo
// -- veja: https://www.gov.br/pf/pt-br/assuntos/passaporte/ajuda/duvidas_/formulario/formulario-nome-completo-nao-cabe
// nome                    VARCHAR(80)     NOT NULL,
// email                   VARCHAR(320)    NOT NULL,
// senha_hash              VARCHAR(64)     NOT NULL,
// url_curriculo_lattes    VARCHAR(200),
// cargo                   cargo_e         NOT NULL    DEFAULT 'aluno',
// registrado_em           TIMESTAMP       NOT NULL    DEFAULT now(),
// atualizado_em           TIMESTAMP,
// desativado_em           TIMESTAMP,
// registro_aluno          VARCHAR(100),
// periodo                 SMALLINT,
