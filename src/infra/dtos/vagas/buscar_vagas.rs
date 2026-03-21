use comum::sqlx::DbDateTime;
use dominio::comum::filtragem::{DirecaoOrdenacao, LimitadorDeData};
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::vagas::filtragem::OrdenacaoDeVaga;
use serde::Deserialize;
use serde_with::{DefaultOnError, serde_as};
use uuid::Uuid;

/// Contém a versão serializada do [`BuscarVagasQueryDto`].
/// Não construa este DTO diretamente, utilize [`BuscarVagasQueryDto::sanitize`] ao invés disso.
///
/// [`BuscarVagasQueryDto`]: BuscarVagasQueryDto
/// [`BuscarVagasQueryDto::sanitize`]: BuscarVagasQueryDto::sanitize
#[derive(Clone)]
pub struct BuscarVagasDto {
    pub(crate) titulo: Option<String>,
    pub(crate) coordenador: Option<Uuid>,
    pub(crate) tipo: Option<TipoDeProjeto>,
    pub(crate) data_de_publicacao: Option<(DbDateTime, LimitadorDeData)>,
    pub(crate) ordenacao: Option<OrdenacaoDeVaga>,
    pub(crate) pagina: Option<u64>,
    pub(crate) qtd_por_pagina: Option<u8>,
}

#[serde_as]
#[derive(Deserialize)]
pub struct BuscarVagasQueryDto {
    #[serde(default)]
    pub titulo: Option<String>,
    #[serde(default)]
    #[serde_with(as = "DefaultOnError")]
    pub coordenador: Option<Uuid>,
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    #[serde_with(as = "DefaultOnError")]
    pub dp_data: Option<DbDateTime>,
    #[serde(default)]
    pub dp_lim: Option<String>,
    #[serde(default)]
    pub ordenar_por: Option<String>,
    #[serde(default)]
    pub direcao_ord: Option<String>,
    #[serde(default)]
    #[serde_as(as = "DefaultOnError")]
    pub pagina: Option<u64>,
    #[serde(default)]
    #[serde_as(as = "DefaultOnError")]
    pub qtd_por_pagina: Option<u8>,
}

impl BuscarVagasQueryDto {
    /// Sanitiza e degrada graciosamente os dados do DTO em um [`BuscarVagasDto`] enriquecido.
    ///
    /// [`BuscarVagasDto`]: BuscarVagasDto
    pub fn sanitize(self) -> BuscarVagasDto {
        let ordenacao = self.processe_ordenador_silenciosamente();
        let tipo_de_projeto = self.processe_tipo_de_projeto_silenciosamente();
        let data_de_publicacao = self.processe_data_de_publicacao();

        BuscarVagasDto {
            titulo: self.titulo,
            coordenador: self.coordenador,
            tipo: tipo_de_projeto,
            data_de_publicacao,
            ordenacao,
            pagina: self.pagina,
            qtd_por_pagina: self.qtd_por_pagina,
        }
    }

    fn processe_data_de_publicacao(&self) -> Option<(DbDateTime, LimitadorDeData)> {
        let limitador_de_data = self
            .dp_lim
            .as_deref()
            .and_then(|limitador| match limitador {
                "ate" => Some(LimitadorDeData::Ate),
                "apos" => Some(LimitadorDeData::Apos),
                _ => None,
            });

        if let (Some(data), Some(limitador)) = (self.dp_data, limitador_de_data) {
            return Some((data, limitador));
        }

        None
    }

    fn processe_tipo_de_projeto_silenciosamente(&self) -> Option<TipoDeProjeto> {
        self.tipo
            .as_ref()
            .and_then(|tipo| match tipo.to_lowercase().as_ref() {
                "extensao" | "extensão" => Some(TipoDeProjeto::Extensao),
                "iniciacao_cientifica"
                | "iniciação_científica"
                | "iniciacaocientifica"
                | "iniciaçãocientífica" => Some(TipoDeProjeto::IniciacaoCientifica),
                _ => None,
            })
    }

    fn processe_ordenador_silenciosamente(&self) -> Option<OrdenacaoDeVaga> {
        let direcao_ordenacao = self
            .direcao_ord
            .as_deref()
            .and_then(|direcao_crua| match direcao_crua {
                "asc" => Some(DirecaoOrdenacao::Asc),
                "desc" => Some(DirecaoOrdenacao::Desc),
                _ => None,
            })
            .unwrap_or_default();

        let ordenador = self
            .ordenar_por
            .as_deref()
            .and_then(|ordenar_por| match ordenar_por {
                "titulo" => Some(OrdenacaoDeVaga::Titulo(direcao_ordenacao)),
                "data" => Some(OrdenacaoDeVaga::Data(direcao_ordenacao)),
                _ => None,
            });

        ordenador
    }
}
