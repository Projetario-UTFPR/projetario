use crate::comum::agregacao_com_coordenador::AgregadoComCoordenador;
use crate::identidade::entidades::professor::Professor;
use crate::identidade::enums::cargo::Cargo;

pub struct PoliticasDeProjetos;

impl PoliticasDeProjetos {
    pub fn professor_tem_poderio_sobre_projeto(
        professor: &Professor,
        projeto: &dyn AgregadoComCoordenador,
    ) -> bool {
        *professor.obtenha_cargo() == Cargo::Administrador
            || projeto
                .obtenha_coordenador()
                .obtenha_usuario()
                .obtenha_id()
                .eq(professor.obtenha_usuario().obtenha_id())
            || projeto.obtenha_vice_coordenador().is_some_and(|vice| {
                vice.obtenha_usuario()
                    .obtenha_id()
                    .eq(professor.obtenha_usuario().obtenha_id())
            })
    }
}
