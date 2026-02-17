use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::vagas::entidades::vaga::Vaga;
use sqlx::{PgPool, QueryBuilder};

pub async fn salvar_vagas(db_conn: &PgPool, vagas: Vec<&Vaga>) {
    let mut query = QueryBuilder::new(
        r#"INSERT INTO vaga (
            id, id_projeto, id_coordenador, id_vice_coordenador, horas_por_semana,
            imagem, quantidade, link_edital, link_candidatura, titulo, conteudo,
            iniciada_em, inscricoes_ate, cancelada_em, atualizada_em
        ) "#,
    );

    query
        .push_values(vagas, |mut b, vaga| {
            b.push_bind(vaga.obtenha_id())
                .push_bind(vaga.obtenha_projeto().obtenha_id())
                .push_bind(vaga.obtenha_coordenador().obtenha_usuario().obtenha_id())
                .push_bind(
                    vaga.obtenha_vice_coordenador()
                        .map(|vice| vice.obtenha_usuario().obtenha_id()),
                )
                .push_bind(vaga.obtenha_horas_por_semana() as i64)
                .push_bind(vaga.obtenha_imagem())
                .push_bind(vaga.obtenha_quantidade() as i64)
                .push_bind(vaga.obtenha_link_edital())
                .push_bind(vaga.obtenha_link_candidatura())
                .push_bind(vaga.obtenha_titulo())
                .push_bind(vaga.obtenha_conteudo())
                .push_bind(vaga.obtenha_data_de_inicio())
                .push_bind(vaga.obtenha_data_final_inscricoes())
                .push_bind(vaga.obtenha_data_de_cancelamento())
                .push_bind(vaga.obtenha_data_de_modificacao());
        })
        .build()
        .execute(db_conn)
        .await
        .expect("Falhou em persistir várias vagas no banco de dados.");
}
