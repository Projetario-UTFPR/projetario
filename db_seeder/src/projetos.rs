use comum::sqlx::db_date_time_now;
use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use dominio::projetos::entidades::projeto::Projeto;
use dominio::projetos::enums::tipo_de_coordenacao::TipoDeCoordenacao;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use log::info;
use sqlx::{PgPool, Postgres, Transaction, query};

use crate::usuarios::UsuariosCriados;

pub async fn inserir_projetos(db_pool: &PgPool, usuarios: &UsuariosCriados) {
    let projetos = vec![
        ProjetoComCoordenadores::novo(
            Projeto::novo(
                "Projetário".into(),
                "Plataforma unificada para busca de projetos de extensão e pesquisas de iniciação científica.".into(),
                TipoDeProjeto::Extensao,
            ),
            usuarios.admin.clone(),
            None,
        ),
        ProjetoComCoordenadores::novo(
            Projeto::novo(
                "AgroTec: Inovação e Sustentabilidade na Agricultura Familiar".into(),
                "<p>Este projeto de extensão visa capacitar pequenos agricultores e agricultoras da região de Campo Mourão sobre o uso de tecnologias sustentáveis para otimizar a produção. Por meio de oficinas, workshops e visitas técnicas, os participantes aprenderão sobre sistemas de irrigação inteligentes, manejo biológico de pragas e técnicas de conservação do solo. O projeto busca, com isso, aumentar a produtividade e a renda das famílias, enquanto promove práticas agrícolas que respeitam o meio ambiente.</p>".into(),
                TipoDeProjeto::Extensao,
            ),
            usuarios.professor.clone(),
            Some(usuarios.admin.clone()),
        ),
        ProjetoComCoordenadores::novo(
            Projeto::novo(
                "Impacto da Poluição por Microplásticos em Ecossistemas Aquáticos Urbanos".into(),
                "<p>A pesquisa tem como objetivo analisar a presença e a concentração de microplásticos em rios e lagos urbanos da região. Por meio da coleta e análise de amostras de água e sedimentos, o estudo pretende identificar as principais fontes de poluição e seus impactos na fauna local. Os resultados da pesquisa serão utilizados para desenvolver materiais educativos e campanhas de conscientização para a população e gestores públicos, buscando a redução da poluição plástica.</p>".into(),
                TipoDeProjeto::IniciacaoCientifica,
            ),
            usuarios.admin.clone(),
            Some(usuarios.professor.clone()),
        ),
    ];

    for projeto in &projetos {
        salvar_projeto(db_pool, projeto).await;
        info!(
            "Inserido projeto {} com o ID {}, coordenador {} e vice-coordenador {}.",
            projeto.obtenha_projeto().obtenha_titulo(),
            projeto.obtenha_projeto().obtenha_id(),
            projeto
                .obtenha_coordenador()
                .obtenha_usuario()
                .obtenha_nome(),
            projeto
                .obtenha_vice_coordenador()
                .map(|vice| vice.obtenha_usuario().obtenha_nome())
                .unwrap_or("NULO")
        )
    }
}

async fn salvar_projeto(db_pool: &PgPool, projeto: &ProjetoComCoordenadores) {
    let mut transaction = db_pool
        .begin()
        .await
        .expect("Não foi possível iniciar uma transação no banco de dados");

    let _ = query(concat!(
        "INSERT INTO projeto ",
        "(id, titulo, descricao, tipo, registrado_em, iniciado_em, atualizado_em, cancelado_em, concluido_em) ",
        "SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9 ",
        "WHERE NOT EXISTS ( SELECT 1 FROM projeto WHERE titulo = $2 )"),
    )
    .bind(projeto.obtenha_projeto().obtenha_id())
    .bind(projeto.obtenha_projeto().obtenha_titulo())
    .bind(projeto.obtenha_projeto().obtenha_descricao())
    .bind(projeto.obtenha_projeto().obtenha_tipo())
    .bind(projeto.obtenha_projeto().obtenha_data_de_registro())
    .bind(projeto.obtenha_projeto().obtenha_data_de_inicio())
    .bind(projeto.obtenha_projeto().obtenha_data_de_modificacao())
    .bind(projeto.obtenha_projeto().obtenha_data_de_cancelamento())
    .bind(projeto.obtenha_projeto().obtenha_data_de_conclusao())
    .execute(&mut *transaction)
    .await;

    let _ = relacionar_com_professor_se_houver(
        &mut transaction,
        projeto.obtenha_projeto(),
        Some(projeto.obtenha_coordenador()),
        TipoDeCoordenacao::Coordenador,
    )
    .await;

    let _ = relacionar_com_professor_se_houver(
        &mut transaction,
        projeto.obtenha_projeto(),
        projeto.obtenha_vice_coordenador(),
        TipoDeCoordenacao::ViceCoordenador,
    )
    .await;

    transaction
        .commit()
        .await
        .expect("Não foi possível registrar o novo projeto no banco de dados");
}

async fn relacionar_com_professor_se_houver(
    db_pool: &mut Transaction<'static, Postgres>,
    projeto: &Projeto,
    coordenador: Option<&Professor>,
    tipo: TipoDeCoordenacao,
) -> Result<(), sqlx::Error> {
    match coordenador {
        Some(coordenador) => query(concat!(
            "INSERT INTO coordenador_projeto ",
            "(id_coordenador, id_projeto, tipo, iniciado_em) ",
            "VALUES ($1, $2, $3, $4)",
        ))
        .bind(coordenador.obtenha_usuario().obtenha_id())
        .bind(projeto.obtenha_id())
        .bind(tipo)
        .bind(db_date_time_now())
        .execute(&mut **db_pool)
        .await
        .map(|_| ()),
        None => Ok(()),
    }
}
