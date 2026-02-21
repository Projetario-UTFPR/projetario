use dominio::comum::filtragem::DirecaoOrdenacao;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::vagas::filtragem::{FiltroDeVaga, OrdenacaoDeVaga};
use serde::Deserialize;
use uuid::Uuid;

/// Contém a versão serializada do [`BuscarVagasQueryDto`].
/// Não construa este DTO diretamente, utilize [`BuscarVagasQueryDto::sanitize`] ao invés disso.
///
/// [`BuscarVagasQueryDto`]: BuscarVagasQueryDto
/// [`BuscarVagasQueryDto::sanitize`]: BuscarVagasQueryDto::sanitize
#[derive(Clone)]
pub struct BuscarVagasDto {
    pub(crate) filtro: Option<FiltroDeVaga>,
    pub(crate) ordenacao: Option<OrdenacaoDeVaga>,
    pub(crate) tipo: Option<TipoDeProjeto>,
    pub(crate) pagina: Option<u64>,
    pub(crate) qtd_por_pagina: Option<u8>,
}

#[derive(Deserialize)]
pub struct BuscarVagasQueryDto {
    #[serde(default)]
    pub filtrar_por: Option<String>,
    #[serde(default)]
    pub filtro: Option<String>,
    #[serde(default)]
    pub ordenar_por: Option<String>,
    #[serde(default)]
    pub direcao_ord: Option<String>,
    #[serde(default)]
    pub tipo: Option<String>,
    #[serde(default)]
    pub pagina: Option<u64>,
    #[serde(default)]
    pub qtd_por_pagina: Option<u8>,
}

impl BuscarVagasQueryDto {
    /// Sanitiza e degrada graciosamente os dados do DTO em um [`BuscarVagasDto`] enriquecido.
    ///
    /// [`BuscarVagasDto`]: BuscarVagasDto
    pub fn sanitize(self) -> BuscarVagasDto {
        let filtro = self.processe_filtro_silenciosamente();
        let ordenacao = self.processe_ordenador_silenciosamente();
        let tipo_de_projeto = self.processe_tipo_de_projeto_silenciosamente();

        BuscarVagasDto {
            filtro,
            ordenacao,
            tipo: tipo_de_projeto,
            pagina: self.pagina,
            qtd_por_pagina: self.qtd_por_pagina,
        }
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

    fn processe_filtro_silenciosamente(&self) -> Option<FiltroDeVaga> {
        if let (Some(filtro), Some(valor)) = (self.filtrar_por.as_deref(), self.filtro.as_deref()) {
            return match filtro {
                "titulo" => Some(FiltroDeVaga::Titulo(valor.into())),
                "coordenador" => match Uuid::try_parse(valor) {
                    Ok(id_coordenador) => Some(FiltroDeVaga::Coordenador(id_coordenador)),
                    Err(_) => None,
                },
                _ => None,
            };
        }

        None
    }
}
