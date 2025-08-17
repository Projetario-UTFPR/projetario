use crate::identidade::entidades::aluno::Aluno;
use crate::identidade::entidades::usuario::builder::UsuarioBuilder;

pub struct AlunoBuilder {
    pub usuario: UsuarioBuilder,
    pub registro_aluno: String,
    pub periodo: u8,
}

impl From<AlunoBuilder> for Aluno {
    fn from(value: AlunoBuilder) -> Self {
        Self {
            periodo: value.periodo,
            registro_aluno: value.registro_aluno,
            usuario: value.usuario.into(),
        }
    }
}
