use chrono::{NaiveDate, NaiveDateTime};

use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::identidade::entidades::usuario::{Usuario, UsuarioModelo};
use crate::dominio::projetos::entidades::projeto::Projeto;
use crate::dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::dominio::vagas::entidades::vaga::Vaga;
use crate::dominio::vagas::repositorios::vaga::RepositorioDeVagas;
use crate::utils::erros::erro_de_dominio::ErroDeDominio;

pub struct CriarVagaParams<'a> {
    pub usuario: &'a UsuarioModelo,
    pub projeto: Projeto,
    pub coordenador: Professor,
    pub vice_coordenador: Option<Professor>,
    pub horas_por_semana: u8,
    pub imagem: Option<String>,
    pub quantidade: u8,
    pub link_edital: String,
    pub conteudo: String,
    pub titulo: String,
    pub link_candidatura: Option<String>,
    pub inscricoes_ate: NaiveDateTime,
    pub iniciada_em: Option<NaiveDate>,
}

pub struct ServicoCriarVaga<RV>
where
    RV: RepositorioDeVagas,
{
    repositorio_de_vagas: RV,
}

impl<RV> ServicoCriarVaga<RV>
where
    RV: RepositorioDeVagas,
{
    pub fn novo(repositorio_de_vagas: RV) -> Self {
        Self {
            repositorio_de_vagas,
        }
    }

    pub async fn executar(&self, params: CriarVagaParams<'_>) -> Result<Vaga, ErroDeDominio> {
        let CriarVagaParams {
            usuario,
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
            iniciada_em,
        } = params;

        let coordenador = match Professor::try_from(usuario) {
            Ok(professor) => professor,
            Err(msg) => {
                return Err(ErroDeDominio::nao_autorizado(
                    "Somente um professor ou um administrador pode criar novas vagas.",
                ));
            }
        };

        let vaga = match iniciada_em {
            None => Vaga::nova(
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
            ),
            Some(data) => Vaga::nova_com_data_de_inicio(
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
                data,
            ),
        };

        self.repositorio_de_vagas.criar_vaga(&vaga).await?;

        Ok(vaga)
    }
}