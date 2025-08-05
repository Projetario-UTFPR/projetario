use projetario::dominio::identidade::entidades::professor::Professor;
use projetario::dominio::projetos::entidades::projeto::Projeto;
use projetario::dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use projetario::utils::sqlx::db_date_time_now;
use sqlx::PgPool;

pub async fn salvar_projeto(db_conn: &PgPool, projeto: &Projeto) {
    sqlx::query(
        r#"INSERT INTO projeto (
            id, titulo, descricao, tipo, registrado_em, iniciado_em,
            atualizado_em, cancelado_em, concluido_em
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
    )
    .bind(projeto.obtenha_id())
    .bind(projeto.obtenha_titulo())
    .bind(projeto.obtenha_descricao())
    .bind(projeto.obtenha_tipo())
    .bind(projeto.obtenha_data_de_registro())
    .bind(projeto.obtenha_data_de_inicio())
    .bind(projeto.obtenha_data_de_modificacao())
    .bind(projeto.obtenha_data_de_cancelamento())
    .bind(projeto.obtenha_data_de_conclusao())
    .execute(db_conn)
    .await
    .expect("Não foi possível salvar o projeto no banco de dados para os testes");
}

pub async fn associar_projeto_com_coordenador(
    db_conn: &PgPool,
    projeto: &Projeto,
    coordenador: &Professor,
    tipo_coordenacao: TipoDeCoordenacao,
) {
    sqlx::query(
        r#"INSERT INTO coordenador_projeto
        (id_coordenador, id_projeto, tipo, iniciado_em)
        VALUES ($1, $2, $3, $4)"#,
    )
    .bind(coordenador.obtenha_usuario().obtenha_id())
    .bind(projeto.obtenha_id())
    .bind(tipo_coordenacao)
    .bind(db_date_time_now())
    .execute(db_conn)
    .await
    .expect(
        "Não foi possível associar o coordenador com o professor no banco de dados para os testes",
    );
}

pub async fn criar_projeto_e_associar(
    db_conn: &PgPool,
    projeto: &Projeto,
    coordenador: &Professor,
    tipo_coordenacao: TipoDeCoordenacao,
) {
    salvar_projeto(db_conn, projeto).await;
    associar_projeto_com_coordenador(db_conn, projeto, coordenador, tipo_coordenacao).await;
}
