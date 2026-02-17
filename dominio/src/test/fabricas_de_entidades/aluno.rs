use rand::Rng;

use crate::identidade::entidades::aluno::{Aluno, AlunoBuilder};
use crate::test::fabricas_de_entidades::usuario_modelo::UsuarioParcial;

#[derive(Default)]
pub struct AlunoParcial {
    usuario: UsuarioParcial,
    registro_aluno: Option<String>,
    periodo: Option<u8>,
}

impl AlunoParcial {
    pub fn into_builder(self) -> AlunoBuilder {
        let mut builder = AlunoBuilder::default();

        builder
            .periodo(
                self.periodo
                    .unwrap_or_else(|| rand::rng().random_range(1..=10)),
            )
            .registro_aluno(
                self.registro_aluno
                    .unwrap_or_else(|| format!("a{:07}", rand::rng().random_range(2..=9999999))),
            )
            .usuario(self.usuario.into_entidade());

        builder
    }

    pub fn into_entidade(self) -> Aluno { self.into_builder().build().unwrap() }
}
