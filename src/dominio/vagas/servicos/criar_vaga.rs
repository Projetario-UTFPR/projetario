use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::identidade::enums::cargo::Cargo;
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::projetos::repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::ResultadoDominio;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;
use crate::utils::sqlx::DbDateTime;

pub struct CriarVagaParams<'this> {
    pub professor: &'this Professor,
    pub id_projeto: Uuid,
    pub horas_por_semana: u8,
    pub imagem: String,
    pub quantidade: u8,
    pub link_edital: String,
    pub conteudo: String,
    pub titulo: Option<String>,
    pub link_candidatura: Option<String>,
    pub inscricoes_ate: DbDateTime,
}

pub struct ServicoCriarVaga<RV, RPC>
where
    RV: RepositorioDeVagas,
    RPC: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_vagas: RV,
    repositorio_de_projetos_e_coords: RPC,
}

impl<RV, RPC> ServicoCriarVaga<RV, RPC>
where
    RV: RepositorioDeVagas,
    RPC: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_vagas: RV, repositorio_de_projetos_e_coords: RPC) -> Self {
        Self {
            repositorio_de_vagas,
            repositorio_de_projetos_e_coords,
        }
    }

    pub async fn executar(
        &self,
        CriarVagaParams {
            conteudo,
            professor,
            horas_por_semana,
            id_projeto,
            imagem,
            inscricoes_ate,
            link_candidatura,
            link_edital,
            quantidade,
            titulo,
        }: CriarVagaParams<'_>,
    ) -> ResultadoDominio<Vaga> {
        let projeto_com_coords = self
            .repositorio_de_projetos_e_coords
            .buscar_projeto_e_coordenadores_por_id(&id_projeto)
            .await?;

        let (projeto, coordenador, vice_coordenador) = projeto_com_coords
            .ok_or_else(|| ErroDeDominio::nao_encontrado("O projeto informado não foi encontrado."))
            .map(|projeto| projeto.desestruture())?;

        if Cargo::Administrador.ne(professor.obtenha_cargo()) && coordenador.ne(professor) {
            return Err(ErroDeDominio::nao_autorizado(
                "Você não tem autorização para abrir vagas para este projeto.",
            ));
        }

        let vaga = Vaga::nova(
            projeto,
            coordenador,
            vice_coordenador,
            horas_por_semana,
            imagem,
            quantidade,
            link_edital,
            conteudo,
            titulo,
            link_candidatura,
            inscricoes_ate,
        )?;

        self.repositorio_de_vagas.criar_vaga(&vaga).await?;

        Ok(vaga)
    }
}
