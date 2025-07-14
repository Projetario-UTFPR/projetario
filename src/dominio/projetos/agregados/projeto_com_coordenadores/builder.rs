use crate::dominio::identidade::entidades::professor::builder::ProfessorBuilder;
use crate::dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::dominio::projetos::entidades::projeto::builder::ProjetoBuilder;

pub struct ProjetoComCoordenadoresBuilder {
    pub projeto: ProjetoBuilder,
    pub coordenador: ProfessorBuilder,
    pub vice_coordenador: Option<ProfessorBuilder>,
}

impl From<ProjetoComCoordenadoresBuilder> for ProjetoComCoordenadores {
    fn from(value: ProjetoComCoordenadoresBuilder) -> Self {
        Self {
            coordenador: value.coordenador.into(),
            projeto: value.projeto.into(),
            vice_coordenador: value.vice_coordenador.map(|vice| vice.into()),
        }
    }
}
