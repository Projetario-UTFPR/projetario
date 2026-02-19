use std::cmp::Reverse;
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
use pretty_assertions::assert_eq;
use projetario::infra::repositorios::sqlx::vagas::RepositorioDeVagasSQLX;
use rstest::rstest;
use sqlx::{PgPool, Postgres};
use uuid::Uuid;

use crate::common::utils::geradores::{gerar_professores, gerar_projetos, gerar_vagas_de_projetos};
use crate::common::utils::gerar_combinacoes;
use crate::common::utils::queries::{
    criar_e_associar_varios_projetos,
    salvar_usuarios,
    salvar_vagas,
};

static ID_DE_COORDENADOR_CONHECIDO: LazyLock<Uuid> = LazyLock::new(Uuid::new_v4);
static TITULO_CONHECIDO: LazyLock<&'static str> = LazyLock::new(|| "Foo Bar");
static DATA_CONHECIDA: LazyLock<DbDateTime> = LazyLock::new(db_date_time_now);

fn assert_vaga_satisfaz_filtros<'a>(vaga: &Vaga, filtros: impl Iterator<Item = &'a FiltroDeVaga>) {
    for filtro in filtros {
        match filtro {
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
        .for_each(|vaga| assert_vaga_satisfaz_filtros(&vaga, [filtro.clone()].iter()));
}

#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_aplicar_intersecao_de_varios_filtros(db_conn: &PgPool) {
    let profs = gerar_professores(NonZeroU8::new(140).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 170, *ID_DE_COORDENADOR_CONHECIDO);

    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        200,
        100,
        100,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo = RepositorioDeVagasSQLX::novo(db_conn);

    let ordenador: OrdenacaoDeVaga = OrdenacaoDeVaga::Data(DirecaoOrdenacao::Asc);
    let paginacao = Paginacao::default();

    let possiveis_filtros = vec![
        FiltroDeVaga::Coordenador(*ID_DE_COORDENADOR_CONHECIDO),
        FiltroDeVaga::Titulo(TITULO_CONHECIDO.to_string()),
        FiltroDeVaga::DataDePublicacao(*DATA_CONHECIDA, LimitadorDeData::Apos),
        FiltroDeVaga::DataDePublicacao(*DATA_CONHECIDA, LimitadorDeData::Ate),
        FiltroDeVaga::Estado(EstadoDaVaga::Ativa),
        FiltroDeVaga::Estado(EstadoDaVaga::Cancelada),
        FiltroDeVaga::Estado(EstadoDaVaga::Encerrada),
        FiltroDeVaga::Tipo(TipoDeProjeto::Extensao),
        FiltroDeVaga::Tipo(TipoDeProjeto::IniciacaoCientifica),
    ];

    for filtros in gerar_combinacoes(possiveis_filtros.as_slice(), 2, 6) {
        let resultado = repo
            .buscar_vagas(filtros.clone(), ordenador.clone(), paginacao.clone())
            .await
            .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

        resultado
            .dados
            .into_iter()
            .for_each(|vaga| assert_vaga_satisfaz_filtros(&vaga, filtros.iter()));
    }
}

fn assert_vagas_estao_ordenadas(vagas: Vec<Vaga>, ordenador: OrdenacaoDeVaga) {
    match ordenador {
        OrdenacaoDeVaga::Data(direcao_ordenacao) => match direcao_ordenacao {
            DirecaoOrdenacao::Asc => {
                assert!(vagas.is_sorted_by_key(|vaga| vaga.obtenha_data_de_inicio()))
            }
            DirecaoOrdenacao::Desc => {
                assert!(vagas.is_sorted_by_key(|vaga| Reverse(vaga.obtenha_data_de_inicio())))
            }
        },
        OrdenacaoDeVaga::Titulo(direcao_ordenacao) => match direcao_ordenacao {
            DirecaoOrdenacao::Asc => assert!(vagas.is_sorted_by_key(|vaga| vaga.obtenha_titulo())),
            DirecaoOrdenacao::Desc => {
                assert!(vagas.is_sorted_by_key(|vaga| Reverse(vaga.obtenha_titulo())))
            }
        },
    }
}

#[with_setup]
#[rstest]
#[case::ordenar_por_titulo_em_ordem_decrescente(OrdenacaoDeVaga::Titulo(DirecaoOrdenacao::Desc))]
#[case::ordenar_por_titulo_em_ordem_crescente(OrdenacaoDeVaga::Titulo(DirecaoOrdenacao::Asc))]
#[case::ordenar_por_data_em_ordem_decrescente(OrdenacaoDeVaga::Data(DirecaoOrdenacao::Desc))]
#[case::ordenar_por_data_em_ordem_crescente(OrdenacaoDeVaga::Data(DirecaoOrdenacao::Asc))]
#[with_db_conn]
#[tokio::test]
async fn deveria_ordenar_corretamente(
    #[ignore] db_conn: &PgPool,
    #[case] ordenacao: OrdenacaoDeVaga,
) {
    let profs = gerar_professores(NonZeroU8::new(10).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 170, *ID_DE_COORDENADOR_CONHECIDO);

    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        100,
        0,
        0,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo = RepositorioDeVagasSQLX::novo(db_conn);

    let resultado = repo
        .buscar_vagas(HashSet::new(), ordenacao.clone(), Paginacao::default())
        .await
        .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

    assert_vagas_estao_ordenadas(resultado.dados, ordenacao);
}

#[with_setup]
#[rstest]
#[with_db_conn]
#[tokio::test]
async fn deveria_limitar_o_limite_de_elementos_por_pagina(#[ignore] db_conn: &PgPool) {
    const TOTAL_DE_VAGAS: u32 = 250;

    let profs = gerar_professores(NonZeroU8::new(10).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 200, *ID_DE_COORDENADOR_CONHECIDO);
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        TOTAL_DE_VAGAS,
        0,
        0,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo = RepositorioDeVagasSQLX::novo(db_conn);

    for qtd in 0..=TOTAL_DE_VAGAS {
        let resultado = repo
            .buscar_vagas(
                HashSet::new(),
                OrdenacaoDeVaga::default(),
                Paginacao::default().com_qtd_por_pagina(qtd as u8),
            )
            .await
            .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

        assert_eq!(qtd as usize, resultado.dados.len());
        assert!((qtd as u64) <= resultado.qtd_total);
    }

    for qtd in (TOTAL_DE_VAGAS + 1)..=255u32 {
        let resultado = repo
            .buscar_vagas(
                HashSet::new(),
                OrdenacaoDeVaga::default(),
                Paginacao::default().com_qtd_por_pagina(qtd as u8),
            )
            .await
            .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

        assert!(qtd as usize > resultado.dados.len());
        assert!((qtd as u64) > resultado.qtd_total);
    }
}

#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_paginar_corretamente_os_dados(db_conn: &PgPool) {
    const TOTAL_DE_VAGAS: u32 = 250;

    let profs = gerar_professores(NonZeroU8::new(10).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 200, *ID_DE_COORDENADOR_CONHECIDO);
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        TOTAL_DE_VAGAS,
        0,
        0,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo: RepositorioDeVagasSQLX<'_> = RepositorioDeVagasSQLX::novo(db_conn);

    const LIMITE_POR_PAGINA: u8 = 15;
    const QTD_DE_PAGINAS_ESPERADAS: u32 = TOTAL_DE_VAGAS.div_ceil(LIMITE_POR_PAGINA as u32);

    let mut vagas_ja_verificadas = Vec::<Vaga>::new();

    for pagina in 1..=QTD_DE_PAGINAS_ESPERADAS {
        let mut resultado = repo
            .buscar_vagas(
                HashSet::new(),
                OrdenacaoDeVaga::default(),
                Paginacao::default()
                    .com_qtd_por_pagina(LIMITE_POR_PAGINA)
                    .com_pagina(pagina as u64),
            )
            .await
            .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

        let vagas_repetidas = vagas_ja_verificadas
            .iter()
            .filter(|vaga_ja_verificada| resultado.dados.contains(*vaga_ja_verificada))
            .collect::<Vec<_>>();

        assert!(
            !resultado.dados.is_empty(),
            "A página retornou vazia, mas este cenário não é esperado"
        );

        let primeira_aparicao_da_vaga_repetida =
            if let Some(primeira_vaga_repetida) = vagas_repetidas.first() {
                let pagina = vagas_ja_verificadas
                    .iter()
                    .position(|vaga| vaga.eq(primeira_vaga_repetida))
                    .unwrap()
                    .div_ceil(LIMITE_POR_PAGINA as usize);

                Some(pagina)
            } else {
                None
            };

        assert!(
            vagas_repetidas.is_empty(),
            "Uma mesma vaga não deveria ser repetida entre páginas diferentes, mas foi encontrada \
            pela primeira vez na página {} e novamente na \
            página {pagina}.\nVagas repetidas: {vagas_repetidas:#?}",
            primeira_aparicao_da_vaga_repetida.unwrap()
        );

        vagas_ja_verificadas.append(&mut resultado.dados);
    }
}

#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_retornar_vazio_se_pagina_estiver_fora_dos_limites(db_conn: &PgPool) {
    const TOTAL_DE_VAGAS: u32 = 40;
    let profs = gerar_professores(NonZeroU8::new(5).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 20, *ID_DE_COORDENADOR_CONHECIDO);
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        TOTAL_DE_VAGAS,
        0,
        0,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo: RepositorioDeVagasSQLX<'_> = RepositorioDeVagasSQLX::novo(db_conn);

    const LIMITE_POR_PAGINA: u8 = 10;
    const ULTIMA_PAGINA: u32 = TOTAL_DE_VAGAS.div_ceil(LIMITE_POR_PAGINA as u32);

    let resultado = repo
        .buscar_vagas(
            HashSet::new(),
            OrdenacaoDeVaga::default(),
            Paginacao::default()
                .com_qtd_por_pagina(LIMITE_POR_PAGINA)
                .com_pagina((ULTIMA_PAGINA + 1) as u64),
        )
        .await
        .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

    assert!(resultado.dados.is_empty());
    assert_eq!(TOTAL_DE_VAGAS as u64, resultado.qtd_total);
}

#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_retornar_vazio_se_nao_houver_vagas_registradas(db_conn: &PgPool) {
    let repo: RepositorioDeVagasSQLX<'_> = RepositorioDeVagasSQLX::novo(db_conn);

    let resultado = repo
        .buscar_vagas(
            HashSet::new(),
            OrdenacaoDeVaga::default(),
            Paginacao::default(),
        )
        .await
        .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

    assert!(resultado.dados.is_empty());
    assert_eq!(0, resultado.qtd_total);
}

#[rstest]
#[case(FiltroDeVaga::Titulo("' OR 1=1 --".to_string()))]
#[case(FiltroDeVaga::Titulo("' UNION SELECT id, email, senha_hash, null, null FROM usuarios --".to_string()))]
#[case(FiltroDeVaga::Titulo("'; DROP TABLE vagas; --".to_string()))]
#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_sanitizar_filtros_maliciosos(
    #[ignore] db_conn: &PgPool,
    #[case] filtro: FiltroDeVaga,
) {
    let profs = gerar_professores(NonZeroU8::new(5).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 20, *ID_DE_COORDENADOR_CONHECIDO);
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        1,
        0,
        0,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo: RepositorioDeVagasSQLX<'_> = RepositorioDeVagasSQLX::novo(db_conn);

    let resultado = repo
        .buscar_vagas(
            HashSet::from_iter([filtro]),
            OrdenacaoDeVaga::default(),
            Paginacao::default(),
        )
        .await
        .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

    assert!(resultado.dados.is_empty());

    let (quantidade_total_de_vagas,) =
        sqlx::query_as::<Postgres, (i64,)>("SELECT COUNT(*) FROM vaga")
            .fetch_one(db_conn)
            .await
            .expect("Falhou em obter a quantidade total de vagas persistidas");

    assert_eq!(
        1, quantidade_total_de_vagas,
        "O banco deveria se manter íntegro após uma query maliciosa tentar ser executada."
    );
}

#[with_setup]
#[with_db_conn]
#[tokio::test]
async fn deveria_retornar_a_quantidade_total_de_vagas_que_satisfazem_o_filtro_sem_considerar_paginacao(
    db_conn: &PgPool,
) {
    const TOTAL_DE_VAGAS_ATIVAS: u32 = 400;
    let profs = gerar_professores(NonZeroU8::new(5).unwrap(), *ID_DE_COORDENADOR_CONHECIDO);
    let usuarios_dos_profs = profs.iter().map(|prof| prof.obtenha_usuario()).collect();
    let projetos = gerar_projetos(profs.iter().collect(), 20, *ID_DE_COORDENADOR_CONHECIDO);
    let vagas = gerar_vagas_de_projetos(
        projetos.iter().collect(),
        TOTAL_DE_VAGAS_ATIVAS,
        255,
        255,
        TITULO_CONHECIDO.to_string(),
        *DATA_CONHECIDA,
    );

    salvar_usuarios(db_conn, usuarios_dos_profs, false).await;
    criar_e_associar_varios_projetos(db_conn, projetos.iter().collect()).await;
    salvar_vagas(db_conn, vagas.iter().collect()).await;

    let repo: RepositorioDeVagasSQLX<'_> = RepositorioDeVagasSQLX::novo(db_conn);

    const LIMITE_POR_PAGINA: u8 = 20;
    const ULTIMA_PAGINA: u8 = (TOTAL_DE_VAGAS_ATIVAS / LIMITE_POR_PAGINA as u32) as u8;

    let paginas = vec![1, ULTIMA_PAGINA, ULTIMA_PAGINA + 1];

    for pagina in paginas {
        let resultado = repo
            .buscar_vagas(
                HashSet::from_iter([FiltroDeVaga::Estado(EstadoDaVaga::Ativa)]),
                OrdenacaoDeVaga::default(),
                Paginacao::default()
                    .com_qtd_por_pagina(LIMITE_POR_PAGINA)
                    .com_pagina(pagina as u64),
            )
            .await
            .expect("Deveria poder realizar uma busca de vagas se os parâmetros estão corretos");

        assert_eq!(
            TOTAL_DE_VAGAS_ATIVAS as u64, resultado.qtd_total,
            "A quantidade de vagas que satisfaz o filtro não deveria ser afetada pela página buscada, \
            mas foi afetado na pagina {pagina} (última página esperada: {ULTIMA_PAGINA})."
        );
    }
}
