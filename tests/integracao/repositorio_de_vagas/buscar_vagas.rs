use std::collections::HashSet;
use std::num::NonZeroU8;
use std::sync::LazyLock;

use app_macros::{with_db_conn, with_setup};
use chrono::{DateTime, Duration, Timelike, Utc};
use comum::sqlx::{DbDateTime, db_date_time_now};
use dominio::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use dominio::comum::filtragem::{DirecaoOrdenacao, LimitadorDeData};
use dominio::comum::paginacao::Paginacao;
use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::test::fabricas_de_entidades::agregados::projeto_com_coordenadores::ProjetoComCoordenadoresParcial;
use dominio::test::fabricas_de_entidades::professor::ProfessorParcial;
use dominio::test::fabricas_de_entidades::projeto::ProjetoParcial;
use dominio::test::fabricas_de_entidades::usuario_modelo::UsuarioParcial;
use dominio::test::fabricas_de_entidades::vaga::VagaParcial;
use dominio::vagas::entidades::vaga::Vaga;
use dominio::vagas::filtragem::{EstadoDaVaga, FiltroDeVaga, OrdenacaoDeVaga};
use dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use fake::Fake;
use fake::faker::boolean::pt_br::Boolean;
use fake::faker::chrono::pt_br::{DateTimeAfter, DateTimeBefore};
use projetario::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;
use rstest::rstest;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

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

fn gerar_usuarios(qtd: NonZeroU8) -> Vec<Professor> {
    let qtd: u8 = qtd.into();
    let mut usuarios = (0..qtd.saturating_sub(1))
        .map(|_| ProfessorParcial::default().into_entidade())
        .collect::<Vec<_>>();

    let professor_conhecido = UsuarioParcial::default()
        .into_builder()
        .id(*ID_DE_COORDENADOR_CONHECIDO)
        .build()
        .unwrap();

    let professor_conhecido = ProfessorParcial::default()
        .into_builder()
        .usuario(professor_conhecido)
        .build()
        .unwrap();

    usuarios.push(professor_conhecido);

    usuarios
}

fn gerar_projetos(coordenadores: Vec<&Professor>, qtd: u8) -> Vec<ProjetoComCoordenadores> {
    if coordenadores.is_empty() {
        return vec![];
    }

    let mut projetos = (0..qtd.saturating_sub(2))
        .map(|_| {
            let idx_coord = (0..coordenadores.len()).fake::<usize>();
            let coordenador = coordenadores[idx_coord].clone();

            let vice_coord = match Boolean(50).fake() {
                true => {
                    let mut idx_vice_coord = (0..coordenadores.len() - 1).fake::<usize>();

                    if idx_vice_coord >= idx_coord {
                        idx_vice_coord += 1;
                    };

                    Some(coordenadores[idx_vice_coord].clone())
                }
                false => None,
            };

            let tipo = match Boolean(50).fake() {
                true => TipoDeProjeto::Extensao,
                false => TipoDeProjeto::IniciacaoCientifica,
            };

            let mut projeto = ProjetoComCoordenadoresParcial::default()
                .into_builder()
                .coordenador(coordenador)
                .vice_coordenador(vice_coord)
                .build()
                .unwrap();

            projeto.obtenha_projeto_mut().coloque_tipo(tipo);

            projeto
        })
        .collect::<Vec<_>>();

    let coordenador_conhecido = coordenadores
        .iter()
        .find(|coord| {
            coord
                .obtenha_usuario()
                .obtenha_id()
                .eq(&*ID_DE_COORDENADOR_CONHECIDO)
        })
        .expect("Não foi possível encontrar um coordenador com o id de coordenador conhecido");

    // Garantia de 1 Projeto de Extensao
    if qtd > 0 {
        let projeto = ProjetoParcial::default()
            .into_builder()
            .tipo(TipoDeProjeto::Extensao)
            .build()
            .unwrap();

        let projeto = ProjetoComCoordenadoresParcial::default()
            .into_builder()
            .coordenador((*coordenador_conhecido).clone())
            .projeto(projeto)
            .build()
            .unwrap();

        projetos.push(projeto);
    }

    // Garantia de 1 Projeto de Iniciação Científica
    if qtd > 1 {
        let projeto = ProjetoParcial::default()
            .into_builder()
            .tipo(TipoDeProjeto::IniciacaoCientifica)
            .build()
            .unwrap();

        let projeto = ProjetoComCoordenadoresParcial::default()
            .into_builder()
            .coordenador((*coordenador_conhecido).clone())
            .projeto(projeto)
            .build()
            .unwrap();

        projetos.push(projeto);
    }

    projetos
}

fn gerar_vagas_de_projetos(
    projetos: Vec<&ProjetoComCoordenadores>,
    ativadas: u8,
    concluidas: u8,
    canceladas: u8,
) -> Vec<Vaga> {
    let mut vagas = Vec::new();

    fn talvez_concatene_titulo_conhecido(vaga: &mut Vaga, p: u8) {
        if Boolean(p).fake() {
            vaga.coloque_titulo(format!("{} {}", vaga.obtenha_titulo(), *TITULO_CONHECIDO));
        }
    }

    for i in 0..ativadas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();
        let data_limite = DateTimeAfter(DATA_CONHECIDA.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc()
            .with_nanosecond(0)
            .unwrap();

        let data_de_inicio = if i % 5 == 0 {
            let data_posterior = DateTimeAfter(DATA_CONHECIDA.and_utc())
                .fake::<DateTime<Utc>>()
                .naive_utc();

            Some(data_posterior + Duration::days(1))
        } else if i % 3 == 0 {
            let data_anterior = DateTimeBefore(DATA_CONHECIDA.and_utc())
                .fake::<DateTime<Utc>>()
                .naive_utc();

            Some(data_anterior - Duration::days(1))
        } else {
            None
        };

        let mut vaga_builder = VagaParcial::default().into_builder();

        vaga_builder
            .projeto_e_coordenadores(projeto)
            .cancelada_em(None)
            .inscricoes_ate(data_limite);

        if let Some(data) = data_de_inicio {
            vaga_builder.iniciada_em(data);
        }

        let mut vaga = vaga_builder.build().unwrap();

        if i == ativadas.saturating_sub(1) {
            talvez_concatene_titulo_conhecido(&mut vaga, 100);
        } else {
            talvez_concatene_titulo_conhecido(&mut vaga, 50);
        }

        vagas.push(vaga);
    }

    for _ in 0..concluidas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();
        let data_limite = DateTimeBefore(DATA_CONHECIDA.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc();

        let mut vaga = VagaParcial::default()
            .into_builder()
            .projeto_e_coordenadores(projeto)
            .cancelada_em(None)
            .inscricoes_ate(data_limite)
            .build()
            .unwrap();

        talvez_concatene_titulo_conhecido(&mut vaga, 50);

        vagas.push(vaga);
    }

    for _ in 0..canceladas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();

        let data_limite = DateTimeAfter(DATA_CONHECIDA.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc()
            .with_nanosecond(0)
            .unwrap();

        let data_cancelamento = fake::faker::chrono::pt_br::DateTime()
            .fake::<DateTime<Utc>>()
            .naive_utc();

        let mut vaga = VagaParcial::default()
            .into_builder()
            .projeto_e_coordenadores(projeto)
            .cancelada_em(data_cancelamento)
            .inscricoes_ate(data_limite)
            .build()
            .unwrap();

        talvez_concatene_titulo_conhecido(&mut vaga, 50);

        vagas.push(vaga);
    }

    vagas
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
    let profs = gerar_usuarios(NonZeroU8::new(5).unwrap());
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 10);

    const TOTAL_DE_VAGAS_CRIADAS: i64 = 30;
    let vagas = gerar_vagas_de_projetos(projetos.iter().collect(), 10, 10, 10);

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
