use crate::dominio::identidade::entidades::professor::Professor;
use crate::dominio::projetos::entidades::projeto::Projeto;

pub mod builder;

#[derive(Debug)]
pub struct ProjetoComCoordenadores {
    projeto: Projeto,
    coordenador: Professor,
    vice_coordenador: Option<Professor>,
}

impl ProjetoComCoordenadores {
    pub fn novo(
        projeto: Projeto,
        coordenador: Professor,
        vice_coordenador: Option<Professor>,
    ) -> Self {
        Self {
            projeto,
            coordenador,
            vice_coordenador,
        }
    }
}

impl ProjetoComCoordenadores {
    pub fn obtenha_projeto(&self) -> &Projeto { &self.projeto }

    pub fn obtenha_coordenador(&self) -> &Professor { &self.coordenador }

    pub fn obtenha_vice_coordenador(&self) -> Option<&Professor> { self.vice_coordenador.as_ref() }
}

impl ProjetoComCoordenadores {
    pub fn desestruture(self) -> (Projeto, Professor, Option<Professor>) {
        (self.projeto, self.coordenador, self.vice_coordenador)
    }
}
