use comum::erros::erro_de_dominio::ErroDeDominio;
use uuid::Uuid;

use crate::identidade::entidades::professor::Professor;
use crate::projetos::politicas::PoliticasDeProjetos;
use crate::vagas::entidades::vaga::Vaga;
use crate::vagas::repositorios::vaga::RepositorioDeVagas;

pub struct ServicoCancelarVaga<RV: RepositorioDeVagas> {
    repositorio: RV,
}

impl<RV> ServicoCancelarVaga<RV>
where
    RV: RepositorioDeVagas,
{
    pub fn novo(repositorio: RV) -> Self { Self { repositorio } }

    pub async fn executar(
        &self,
        vaga_id: Uuid,
        professor: &Professor,
    ) -> Result<Vaga, ErroDeDominio> {
        let mut vaga = self.repositorio.buscar_por_id(&vaga_id).await?.ok_or(
            ErroDeDominio::nao_encontrado("Vaga não encontrada".to_string()),
        )?;

        if !vaga.esta_ativa() {
            return Err(ErroDeDominio::integridade(
                "Não é possível cancelar uma vaga que não está mais ativa.",
            ));
        }

        let professor_pode_alterar_vaga =
            PoliticasDeProjetos::professor_tem_poderio_sobre_projeto(professor, &vaga);

        if !professor_pode_alterar_vaga {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem permissão para cancelar esta vaga".to_string(),
            ));
        }

        vaga.cancelar();
        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}

// TODO: implementar testes unitários para o serviço de cancelar vaga
