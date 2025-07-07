use chrono::{NaiveDate, NaiveDateTime, Utc};
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

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
    pub fn novo(repositorio: RV) -> Self { Self { repositorio } }

    pub async fn executar(
        &self,
        vaga_id: Uuid,
        params: AlterarVagaParams,
        professor: &Professor,
    ) -> Result<Vaga, ErroDeDominio> {
        let mut vaga =
            self.repositorio
                .buscar_por_id(vaga_id)
                .await?
                .ok_or(ErroDeDominio::nao_encontrado(
                    "Vaga não encontrada".to_string(),
                ))?;

        let professor_pode_alterar_vaga = vaga
            .obtenha_coordenador()
            .obtenha_usuario()
            .obtenha_id()
            .eq(professor.obtenha_usuario().obtenha_id())
            || *professor.obtenha_cargo() == Cargo::Administrador;

        if !professor_pode_alterar_vaga {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem permissão para alterar esta vaga",
            ));
        }

        if let Some(horas) = params.horas_por_semana {
            vaga.definir_horas_por_semana(horas)?;
        }

        if let Some(imagem) = params.imagem {
            vaga.coloque_imagem(imagem);
        }

        if let Some(quantidade) = params.quantidade {
            vaga.definir_quantidade(quantidade)?;
        }

        if let Some(link_edital) = params.link_edital {
            vaga.definir_link_edital(link_edital);
        }

        if let Some(conteudo) = params.conteudo {
            vaga.definir_conteudo(conteudo)?;
        }

        if let Some(titulo) = params.titulo {
            vaga.definir_titulo(titulo)?;
        }

        if let Some(link_candidatura) = params.link_candidatura {
            vaga.definir_link_candidatura(link_candidatura);
        }

        if let Some(inscricoes_ate) = params.inscricoes_ate {
            vaga.definir_inscricoes_ate(inscricoes_ate)?;
        }

        if let Some(iniciada_em) = params.iniciada_em {
            vaga.definir_iniciada_em(iniciada_em)?;
        }

        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}
