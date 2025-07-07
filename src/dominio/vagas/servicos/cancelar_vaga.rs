use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::UsuarioModelo;
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub struct ServicoCancelarVaga<RV> {
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
                "Você não tem permissão para cancelar esta vaga".to_string(),
            ));
        }

        vaga.cancelar()?;
        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}
