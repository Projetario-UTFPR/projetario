use chrono::NaiveDate;

use crate::{
    dominio::{
        identidade::entidades::{
            professor::Professor,
            usuario::{Usuario, UsuarioModelo},
        },
        projetos::{
            entidades::projeto::Projeto, enums::tipo_de_projeto::TipoDeProjeto,
            repositorios::coordenadores_de_projetos::RepositorioDeCoordenadoresDeProjetos,
        },
    },
    utils::erros::erro_de_dominio::ErroDeDominio,
};

pub struct CriarProjetosDeExtensaoParams<'a> {
    pub usuario: &'a UsuarioModelo,
    pub titulo: String,
    pub descricao: String,
    pub data_de_inicio: Option<NaiveDate>,
}

pub struct ServicoCriarProjetoDeExtensao<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    repositorio_de_coordenadores: RCP,
}

impl<RCP> ServicoCriarProjetoDeExtensao<RCP>
where
    RCP: RepositorioDeCoordenadoresDeProjetos,
{
    pub fn novo(repositorio_de_coordenadores: RCP) -> Self {
        Self {
            repositorio_de_coordenadores,
        }
    }

    pub async fn executar(
        &self,
        params: CriarProjetosDeExtensaoParams<'_>,
    ) -> Result<Projeto, ErroDeDominio> {
        let CriarProjetosDeExtensaoParams {
            usuario,
            descricao,
            titulo,
            data_de_inicio,
        } = params;

        let professor = match Professor::try_from(usuario) {
            Ok(professor) => professor,
            Err(msg) => {
                return Err(ErroDeDominio::nao_autorizado(
                    "Somente um professor ou um administrador pode criar um novo projeto de extensão.",
                ));
            }
        };

        let projeto = match data_de_inicio {
            None => Projeto::novo(titulo, descricao, TipoDeProjeto::Extensao),
            Some(data) => {
                Projeto::novo_com_data_de_inicio(titulo, descricao, TipoDeProjeto::Extensao, data)
            }
        };

        self.repositorio_de_coordenadores
            .criar_projeto_com_coordenador(&projeto, &professor)
            .await?;

        Ok(projeto)
    }
}
