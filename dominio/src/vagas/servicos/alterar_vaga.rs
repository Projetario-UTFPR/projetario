use comum::erros::erro_de_dominio::ErroDeDominio;
use comum::sqlx::DbDateTime;
use uuid::Uuid;

use crate::identidade::entidades::professor::Professor;
use crate::projetos::politicas::PoliticasDeProjetos;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::repositorios::vaga::RepositorioDeVagas;

pub struct AlterarVagaParams {
    pub horas_por_semana: Option<u8>,
    pub imagem: Option<String>,
    pub quantidade: Option<u8>,
    pub link_edital: Option<String>,
    pub conteudo: Option<String>,
    pub titulo: Option<String>,
    pub link_candidatura: Option<Option<String>>,
    pub inscricoes_ate: Option<DbDateTime>,
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
        let mut vaga = self.repositorio.buscar_por_id(&vaga_id).await?.ok_or(
            ErroDeDominio::nao_encontrado("Vaga não encontrada".to_string()),
        )?;

        let professor_pode_alterar_vaga =
            PoliticasDeProjetos::professor_tem_poderio_sobre_projeto(professor, &vaga);

        if !professor_pode_alterar_vaga {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem permissão para alterar esta vaga",
            ));
        }

        if let Some(horas) = params.horas_por_semana {
            vaga.coloque_horas_por_semana(horas)?;
        }

        if let Some(imagem) = params.imagem {
            vaga.coloque_imagem(imagem);
        }

        if let Some(quantidade) = params.quantidade {
            vaga.coloque_quantidade_de_vagas(quantidade)?;
        }

        if let Some(link_edital) = params.link_edital {
            vaga.coloque_link_edital(link_edital);
        }

        if let Some(conteudo) = params.conteudo {
            vaga.coloque_conteudo(conteudo);
        }

        if let Some(titulo) = params.titulo {
            vaga.coloque_titulo(titulo);
        }

        if let Some(link_candidatura) = params.link_candidatura {
            vaga.coloque_link_candidatura(link_candidatura);
        }

        if let Some(inscricoes_ate) = params.inscricoes_ate {
            vaga.atualize_data_de_encerramento_das_inscricoes(inscricoes_ate)?;
        }

        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}
