use actix_web::web::{Data, Query};
use actix_web::{HttpRequest, web};
use dominio::comum::paginacao::Paginacao;
use dominio::vagas::filtragem::EstadoDaVaga;
use dominio::vagas::servicos::buscar_vagas::{
    BuscarVagasDeProjetosParams,
    PossiveisFiltrosParaBuscarVagas,
};
use inertia_rust::{Inertia, InertiaError, InertiaFacade, InertiaProp, hashmap, prop_resolver};
use sqlx::PgPool;

use crate::infra::dtos::vagas::buscar_vagas::{BuscarVagasDto, BuscarVagasQueryDto};
use crate::infra::fabricas::servicos::buscar_vagas::obtenha_servico_buscar_vagas;
use crate::infra::http::RouterRegistrable;
use crate::infra::http::controllers::RespostaDoApp;
use crate::infra::http::presenters::paginacao::PaginacaoMelhoradaPresenter;
use crate::infra::http::presenters::preview_de_vaga::PreviewDeVagaPresenter;
use crate::libs::inertia::inertiafy_domain_error;

pub struct ControllerVagas;

impl RouterRegistrable for ControllerVagas {
    fn register(cfg: &mut web::ServiceConfig) {
        cfg.service(web::scope("/vagas").route("", web::get().to(Self::listar)));
    }
}

impl ControllerVagas {
    async fn listar(
        req: HttpRequest,
        db_conn: Data<PgPool>,
        buscar_vagas_dto: Query<BuscarVagasQueryDto>,
    ) -> RespostaDoApp {
        let buscar_vagas_dto = buscar_vagas_dto.into_inner().sanitize();

        Inertia::render_with_props(
            &req,
            "vagas/index".into(),
            hashmap![
                 "vagas" => InertiaProp::Deferred(prop_resolver!(
                    let db_conn = db_conn.clone(),
                    let buscar_vagas_dto = buscar_vagas_dto.clone();
                    { obtenha_propriedade_vagas(buscar_vagas_dto, db_conn).await }
                ), None)
            ],
        )
        .await
        .map_err(Into::into)
    }
}

async fn obtenha_propriedade_vagas(
    buscar_vagas_dto: BuscarVagasDto,
    db_conn: Data<PgPool>,
) -> Result<serde_json::Value, InertiaError> {
    let servico_buscar_vagas = obtenha_servico_buscar_vagas(&db_conn);

    let vagas = servico_buscar_vagas
        .executar(BuscarVagasDeProjetosParams {
            ordenador: buscar_vagas_dto.ordenacao,
            paginacao: Some(Paginacao::nova_por_opcionais(
                buscar_vagas_dto.pagina,
                buscar_vagas_dto.qtd_por_pagina,
            )),
            possiveis_filtros: PossiveisFiltrosParaBuscarVagas {
                titulo: buscar_vagas_dto.titulo,
                tipo: buscar_vagas_dto.tipo,
                coordenador: buscar_vagas_dto.coordenador,
                data_de_publicacao: buscar_vagas_dto.data_de_publicacao,
                estado: Some(EstadoDaVaga::Ativa),
            },
        })
        .await
        .map(|vagas_paginadas| {
            PaginacaoMelhoradaPresenter::apresente(
                &vagas_paginadas,
                PreviewDeVagaPresenter::apresente,
            )
        });

    inertiafy_domain_error(vagas)
}
