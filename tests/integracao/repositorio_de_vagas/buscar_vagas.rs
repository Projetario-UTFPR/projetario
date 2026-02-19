use std::collections::HashSet;
use std::num::NonZeroU8;
use std::sync::LazyLock;

use app_macros::{with_db_conn, with_setup};
use comum::sqlx::{DbDateTime, db_date_time_now};
use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::comum::filtragem::{DirecaoOrdenacao, LimitadorDeData};
use dominio::comum::paginacao::Paginacao;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::vagas::entidades::vaga::Vaga;
use dominio::vagas::filtragem::{EstadoDaVaga, FiltroDeVaga, OrdenacaoDeVaga};
use dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use projetario::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;
use rstest::rstest;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::common::utils::geradores::{gerar_professores, gerar_projetos, gerar_vagas_de_projetos};
use crate::common::utils::queries::{
    criar_e_associar_varios_projetos,
    salvar_usuarios,
    salvar_vagas,
};

static ID_DE_COORDENADOR_CONHECIDO: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4);
static TITULO_CONHECIDO: LazyLock<&'static str> = LazyLock::new(|| "Foo Bar");
static DATA_CONHECIDA: LazyLock<DbDateTime> = LazyLock::new(db_date_time_now);

fn assert_satisfaz_a_vaga(vaga: &Vaga, filtro: &FiltroDeVaga) {
    match &filtro {
        FiltroDeVaga::Titulo(titulo) => assert!(
            vaga.obtenha_titulo()
                .to_lowercase()
                .contains(&titulo.to_lowercase())
        ),
        FiltroDeVaga::Tipo(tipo_de_projeto) => {
            assert!(vaga.obtenha_projeto().obtenha_tipo().eq(tipo_de_projeto));
        }
        FiltroDeVaga::Coordenador(id_do_coordenador) => {
            let coordenador = vaga.obtenha_coordenador().obtenha_usuario();
            let vice_coord = vaga
                .obtenha_vice_coordenador()
                .map(|vice| vice.obtenha_usuario());

            assert!(
                id_do_coordenador.eq(coordenador.obtenha_id())
                    || vice_coord.is_some_and(|vice| id_do_coordenador.eq(vice.obtenha_id()))
            );
        }
        FiltroDeVaga::DataDePublicacao(data, limitador_de_data) => match limitador_de_data {
            LimitadorDeData::Apos => assert!(vaga.obtenha_data_de_inicio() > data.date()),
            LimitadorDeData::Ate => assert!(vaga.obtenha_data_de_inicio() <= data.date()),
        },
        FiltroDeVaga::Estado(estado_da_vaga) => match estado_da_vaga {
            EstadoDaVaga::Ativa => assert!(vaga.esta_ativa()),
            EstadoDaVaga::Cancelada => assert!(vaga.foi_cancelada()),
            EstadoDaVaga::Encerrada => assert!(vaga.foi_concluida()),
        },
    }
}

#[with_setup]
#[rstest]
#[case::filtrar_por_coordenador(FiltroDeVaga::Coordenador(*ID_DE_COORDENADOR_CONHECIDO), 1)]
#[case::filtrar_por_titulo(FiltroDeVaga::Titulo(TITULO_CONHECIDO.to_string()), 1)]
#[case::filtrar_por_data_apos(FiltroDeVaga::DataDePublicacao(*DATA_CONHECIDA, LimitadorDeData::Apos), 1)]
#[case::filtrar_por_data_ate(FiltroDeVaga::DataDePublicacao(*DATA_CONHECIDA, LimitadorDeData::Ate), 1)]
#[case::filtrar_por_estado_ativo(FiltroDeVaga::Estado(EstadoDaVaga::Ativa), 10)]
#[case::filtrar_por_estado_cancelado(FiltroDeVaga::Estado(EstadoDaVaga::Cancelada), 10)]
#[case::filtrar_por_estado_encerrado(FiltroDeVaga::Estado(EstadoDaVaga::Encerrada), 10)]
#[case::filtrar_por_tipo_extensao(FiltroDeVaga::Tipo(TipoDeProjeto::Extensao), 1)]
#[case::filtrar_por_tipo_iniciacao_cientifica(
    FiltroDeVaga::Tipo(TipoDeProjeto::IniciacaoCientifica),
    1
)]
#[awt]
#[with_db_conn]
#[tokio::test]
async fn deveria_aplicar_corretamente_cada_filtro_possivel(
    #[case] filtro: FiltroDeVaga,
    #[case] minimo_esperado: u8,
    #[ignore] db_conn: &PgPool,
) {
    let profs = gerar_professores(NonZeroU8::new(5).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 10, *ID_DE_COORDENADOR_CONHECIDO);

    const TOTAL_DE_VAGAS_CRIADAS: i64 = 30;
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        10,
        10,
        10,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo = RepositorioDeVagasSQLX::novo(db_conn);

    let filtros = HashSet::from_iter(vec![filtro.clone()]);
    let ordenador: OrdenacaoDeVaga = OrdenacaoDeVaga::Data(DirecaoOrdenacao::Asc);
    let paginacao = Paginacao::default();

    let resultado = repo
        .buscar_vagas(filtros, ordenador, paginacao)
        .await
        .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

    let (quantidade_total_de_vagas,) =
        sqlx::query_as::<Postgres, (i64,)>("SELECT COUNT(*) FROM vaga")
            .fetch_one(db_conn)
            .await
            .expect("Falhou em obter a quantidade total de vagas persistidas");

    assert_eq!(TOTAL_DE_VAGAS_CRIADAS, quantidade_total_de_vagas);

    assert!(minimo_esperado as u64 <= resultado.qtd_total);

    resultado
        .dados
        .into_iter()
        .for_each(|vaga| assert_satisfaz_a_vaga(&vaga, &filtro));
}
