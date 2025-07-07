use uuid::Uuid;
use chrono::{NaiveDate, NaiveDateTime, Utc};

use crate::{
    dominio::{
        identidade::entidades::{professor::Professor, usuario::UsuarioModelo},
        vagas::{
            entidades::vaga::Vaga,
            repositorios::vaga::RepositorioDeVagas,
        },
    },
    utils::erros::erro_de_dominio::ErroDeDominio,
};

pub struct AlterarVagaParams {
    pub horas_por_semana: Option<u8>,
    pub imagem: Option<Option<String>>,
    pub quantidade: Option<u8>,
    pub link_edital: Option<String>,
    pub conteudo: Option<String>,
    pub titulo: Option<String>,
    pub link_candidatura: Option<Option<String>>,
    pub inscricoes_ate: Option<NaiveDateTime>,
    pub iniciada_em: Option<NaiveDate>,
}

pub struct ServicoAlterarVaga<RV> {
    repositorio: RV,
}

impl<RV> ServicoAlterarVaga<RV>
where
    RV: RepositorioDeVagas,
{
    pub fn novo(repositorio: RV) -> Self {
        Self { repositorio }
    }

    pub async fn executar(
        &self,
        vaga_id: Uuid,
        params: AlterarVagaParams,
        usuario: &UsuarioModelo,
    ) -> Result<Vaga, ErroDeDominio> {
        let mut vaga = self.repositorio.buscar_por_id(vaga_id).await?
            .ok_or(ErroDeDominio::nao_encontrado("Vaga não encontrada".to_string()))?;

        let professor = Professor::try_from(usuario)
            .map_err(|_| ErroDeDominio::nao_autorizado("Apenas professores podem alterar vagas".to_string()))?;

        if !vaga.permite_edicao_por(&professor) {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem permissão para alterar esta vaga".to_string(),
            ));
        }

        if let Some(horas) = params.horas_por_semana {
            if horas == 0 || horas > 40 {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Horas por semana devem estar entre 1 e 40".to_string(),
                ));
            }
            vaga.definir_horas_por_semana(horas)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(imagem) = params.imagem {
            vaga.imagem = imagem;
        }

        if let Some(quantidade) = params.quantidade {
            if quantidade == 0 {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Quantidade deve ser pelo menos 1".to_string(),
                ));
            }
            vaga.definir_quantidade(quantidade)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(link_edital) = params.link_edital {
            if link_edital.is_empty() {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Link do edital não pode ser vazio".to_string(),
                ));
            }
            vaga.definir_link_edital(link_edital)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(conteudo) = params.conteudo {
            if conteudo.is_empty() {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Conteúdo não pode ser vazio".to_string(),
                ));
            }
            vaga.definir_conteudo(conteudo)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(titulo) = params.titulo {
            if titulo.is_empty() {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Título não pode ser vazio".to_string(),
                ));
            } else if titulo.len() > 100 {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Título não pode exceder 100 caracteres".to_string(),
                ));
            }
            vaga.definir_titulo(titulo)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(link_candidatura) = params.link_candidatura {
            vaga.definir_link_candidatura(link_candidatura);
        }

        if let Some(inscricoes_ate) = params.inscricoes_ate {
            if inscricoes_ate < Utc::now().naive_utc() {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Data de fechamento de inscrições não pode ser no passado".to_string(),
                ));
            }
            vaga.definir_inscricoes_ate(inscricoes_ate)
            .map_err(ErroDeDominio::regra_de_negocio)?;
        }

        if let Some(iniciada_em) = params.iniciada_em {
            if iniciada_em < Utc::now().date_naive() {
                return Err(ErroDeDominio::regra_de_negocio(
                    "Data de início não pode ser no passado".to_string(),
                ));
            }
            vaga.definir_iniciada_em(iniciada_em)
                .map_err(ErroDeDominio::regra_de_negocio)?;
        }


        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}