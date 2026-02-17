use derive_builder::Builder;

use crate::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use crate::identidade::entidades::professor::Professor;
use crate::projetos::entidades::projeto::Projeto;

#[derive(Debug, Clone, Builder)]
#[builder(setter(into))]
pub struct ProjetoComCoordenadores {
    projeto: Projeto,
    coordenador: Professor,
    #[builder(default = None)]
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
    pub fn obtenha_projeto_mut(&mut self) -> &mut Projeto { &mut self.projeto }
}

impl AgregadoComCoordenador for ProjetoComCoordenadores {
    fn obtenha_coordenador(&self) -> &Professor { &self.coordenador }

    fn obtenha_vice_coordenador(&self) -> Option<&Professor> { self.vice_coordenador.as_ref() }
}

impl ProjetoComCoordenadores {
    pub fn desestruture(self) -> (Projeto, Professor, Option<Professor>) {
        (self.projeto, self.coordenador, self.vice_coordenador)
    }
}
