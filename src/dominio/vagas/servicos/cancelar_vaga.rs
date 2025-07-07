use uuid::Uuid;
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

pub struct ServicoCancelarVaga<RV> {
    repositorio: RV,
}

impl<RV> ServicoCancelarVaga<RV>
where
    RV: RepositorioDeVagas,
{
    pub fn novo(repositorio: RV) -> Self {
        Self { repositorio }
    }

    pub async fn executar(
        &self,
        vaga_id: Uuid,
        usuario: &UsuarioModelo,
    ) -> Result<Vaga, ErroDeDominio> {
        let mut vaga = self.repositorio.buscar_por_id(vaga_id).await?
            .ok_or(ErroDeDominio::nao_encontrado("Vaga não encontrada".to_string()))?;

        let professor = Professor::try_from(usuario)
            .map_err(|_| ErroDeDominio::nao_autorizado("Apenas professores podem cancelar vagas".to_string()))?;

        if !vaga.permite_edicao_por(&professor) {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem permissão para cancelar esta vaga".to_string(),
            ));
        }

        if vaga.concluida_em().is_some() {
            return Err(ErroDeDominio::regra_de_negocio(
                "Não é possível cancelar uma vaga concluída".to_string(),
            ));
        }

        vaga.cancelar()
            .map_err(ErroDeDominio::regra_de_negocio)?;

        self.repositorio.atualizar_vaga(&vaga).await?;

        Ok(vaga)
    }
}