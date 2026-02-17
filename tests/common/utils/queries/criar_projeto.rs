use comum::sqlx::db_date_time_now;
use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use dominio::projetos::entidades::projeto::Projeto;
use dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use sqlx::{PgPool, QueryBuilder};

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

pub async fn criar_e_associar_varios_projetos(
    db_conn: &PgPool,
    projetos: Vec<&ProjetoComCoordenadores>,
) {
    let mut query_projetos = QueryBuilder::new(
        r#"INSERT INTO projeto (
            id, titulo, descricao, tipo, registrado_em, iniciado_em,
            atualizado_em, cancelado_em, concluido_em
        ) "#,
    );

    let mut query_associacoes = QueryBuilder::new(
        "INSERT INTO coordenador_projeto (id_coordenador, id_projeto, tipo, iniciado_em) ",
    );

    query_projetos
        .push_values(&projetos, |mut b, projeto| {
            let projeto = projeto.obtenha_projeto();
            b.push_bind(projeto.obtenha_id())
                .push_bind(projeto.obtenha_titulo())
                .push_bind(projeto.obtenha_descricao())
                .push_bind(projeto.obtenha_tipo())
                .push_bind(projeto.obtenha_data_de_registro())
                .push_bind(projeto.obtenha_data_de_inicio())
                .push_bind(projeto.obtenha_data_de_modificacao())
                .push_bind(projeto.obtenha_data_de_cancelamento())
                .push_bind(projeto.obtenha_data_de_conclusao());
        })
        .build()
        .execute(db_conn)
        .await
        .expect("Não foi possível salvar vários projetos no banco de dados para os testes");

    let mut associacoes = Vec::new();

    projetos.iter().for_each(|projeto_e_coords| {
        associacoes.push((
            projeto_e_coords
                .obtenha_coordenador()
                .obtenha_usuario()
                .obtenha_id(),
            projeto_e_coords.obtenha_projeto().obtenha_id(),
            TipoDeCoordenacao::Coordenador,
        ));

        if let Some(vice) = projeto_e_coords.obtenha_vice_coordenador() {
            associacoes.push((
                vice.obtenha_usuario().obtenha_id(),
                projeto_e_coords.obtenha_projeto().obtenha_id(),
                TipoDeCoordenacao::ViceCoordenador,
            ));
        }
    });

    query_associacoes
        .push_values(associacoes, |mut b, associacao| {
            let (id_coordenador, id_projeto, tipo_coordenacao) = associacao;
            b.push_bind(id_coordenador)
                .push_bind(id_projeto)
                .push_bind(tipo_coordenacao)
                .push_bind(db_date_time_now());
        })
        .build()
        .execute(db_conn)
        .await
        .expect("Não foi possível associar vários coordenadores com projetos no banco de dados para os testes");
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
