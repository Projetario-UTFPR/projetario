use crate::identidade::entidades::professor::Professor;
use crate::projetos::agregados::projeto_com_coordenadores::{
    ProjetoComCoordenadores,
    ProjetoComCoordenadoresBuilder,
};
use crate::projetos::entidades::projeto::Projeto;
use crate::test::fabricas_de_entidades::professor::ProfessorParcial;
use crate::test::fabricas_de_entidades::projeto::ProjetoParcial;

#[derive(Default)]
pub struct ProjetoComCoordenadoresParcial {
    pub projeto: Option<Projeto>,
    pub coordenador: Option<Professor>,
    pub vice_coordenador: Option<Professor>,
}

impl ProjetoComCoordenadoresParcial {
    pub fn into_builder(self) -> ProjetoComCoordenadoresBuilder {
        let mut builder = ProjetoComCoordenadoresBuilder::default();

        builder
            .projeto(
                self.projeto
                    .unwrap_or_else(|| ProjetoParcial::default().into_entidade()),
            )
            .coordenador(
                self.coordenador
                    .unwrap_or_else(|| ProfessorParcial::default().into_entidade()),
            );

        if let Some(vice) = self.vice_coordenador {
            builder.vice_coordenador(vice);
        }

        builder
    }

    pub fn into_entidade(self) -> ProjetoComCoordenadores { self.into_builder().build().unwrap() }
}
