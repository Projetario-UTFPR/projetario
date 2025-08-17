use dominio::identidade::entidades::aluno::Aluno;
use dominio::identidade::entidades::professor::Professor;
use dominio::identidade::entidades::usuario::UsuarioModelo;
use sqlx::{PgPool, query_as};

use crate::senhas::aplicar_hash;

#[allow(dead_code)]
pub struct UsuariosCriados {
    pub admin: Professor,
    pub professor: Professor,
    pub aluno: Aluno,
}

pub async fn inserir_usuarios(db_pool: &PgPool) -> UsuariosCriados {
    let criar_professor = query_as(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo) \
        SELECT 'Reginaldo Ré', 'reginaldo@utfpr.com', $1, 'professor' \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE email = 'reginaldo@utfpr.com' )
        RETURNING *",
    )
    .bind(aplicar_hash("12345"))
    .fetch_one(db_pool);

    let criar_administrador = query_as(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo) \
        SELECT 'Paulo Sabo', 'cremoso@utfpr.com', $1, 'administrador' \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE email = 'cremoso@utfpr.com' )
        RETURNING *",
    )
    .bind(aplicar_hash("12345"))
    .fetch_one(db_pool);

    const RA_ALUNO: &str = "a2250331";

    let criar_aluno = query_as(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo, registro_aluno, periodo) \
        SELECT 'Pedro Alberto', 'pedroalberto@alunos.utfpr.com', $1, 'aluno', $2, 2 \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE registro_aluno = $2 )
        RETURNING *",
    )
    .bind(aplicar_hash("12345"))
    .bind(RA_ALUNO)
    .fetch_one(db_pool);

    let (prof, admin, aluno): (UsuarioModelo, UsuarioModelo, UsuarioModelo) =
        match tokio::try_join!(criar_professor, criar_administrador, criar_aluno) {
            Err(err) => panic!("{err}"),
            Ok(result) => result,
        };

    log::info!(
        "Adicionado o professor Reginaldo Ré com as credenciais: reginaldo@utfpr.com, 12345"
    );

    log::info!(
        "Adicionado o administrador Paulo Sabo com as credenciais: cremoso@utfpr.com, 12345"
    );

    log::info!(
        "Adicionado o aluno Pedro Alberto com as credenciais: pedroalberto@alunos.utfpr.com, 12345, {RA_ALUNO}"
    );

    UsuariosCriados {
        admin: Professor::try_from(&admin).unwrap(),
        professor: Professor::try_from(&prof).unwrap(),
        aluno: Aluno::try_from(&aluno).unwrap(),
    }
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
