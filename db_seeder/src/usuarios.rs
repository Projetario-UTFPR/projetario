use sqlx::{PgPool, query};

use crate::senhas::aplicar_hash;

pub async fn inserir_usuarios(db_pool: &PgPool) {
    let criar_professor = query(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo) \
        SELECT 'Reginaldo Ré', 'reginaldo@utfpr.com', $1, 'professor' \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE email = 'reginaldo@utfpr.com' )",
    )
    .bind(aplicar_hash("12345"))
    .execute(db_pool);

    let criar_administrador = query(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo) \
        SELECT 'Paulo Sabo', 'cremoso@utfpr.com', $1, 'administrador' \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE email = 'cremoso@utfpr.com' )",
    )
    .bind(aplicar_hash("12345"))
    .execute(db_pool);

    let criar_aluno = query(
        "INSERT INTO usuario \
        (nome, email, senha_hash, cargo, registro_aluno, periodo) \
        SELECT 'Pedro Alberto', 'pedroalberto@alunos.utfpr.com', $1, 'aluno', 'a2250331', 2 \
        WHERE NOT EXISTS ( SELECT 1 FROM usuario WHERE registro_aluno = 'a2250331' )",
    )
    .bind(aplicar_hash("12345"))
    .execute(db_pool);

    if let Err(err) = tokio::try_join!(criar_professor, criar_administrador, criar_aluno) {
        panic!("{err}");
    };

    log::info!(
        "Adicionado o professor Reginaldo Ré com as credenciais: reginaldo@utfpr.com, 12345"
    );

    log::info!(
        "Adicionado o administrador Paulo Sabo com as credenciais: cremoso@utfpr.com, 12345"
    );

    log::info!(
        "Adicionado o aluno Pedro Alberto com as credenciais: pedroalberto@alunos.utfpr.com, 12345"
    );
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
