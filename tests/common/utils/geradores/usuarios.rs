use std::num::NonZeroU8;

use dominio::identidade::entidades::professor::Professor;
use dominio::test::fabricas_de_entidades::professor::ProfessorParcial;
use dominio::test::fabricas_de_entidades::usuario_modelo::UsuarioParcial;
use uuid::Uuid;

/// Gera `qtd` professores, sendo 1 deles portador do ID `id_conhecido`.
pub fn gerar_professores(qtd: NonZeroU8, id_conhecido: Uuid) -> Vec<Professor> {
    let qtd: u8 = qtd.into();
    let mut usuarios = (0..qtd.saturating_sub(1))
        .map(|_| ProfessorParcial::default().into_entidade())
        .collect::<Vec<_>>();

    let professor_conhecido = UsuarioParcial::default()
        .into_builder()
        .id(id_conhecido)
        .build()
        .unwrap();

    let professor_conhecido = ProfessorParcial::default()
        .into_builder()
        .usuario(professor_conhecido)
        .build()
        .unwrap();

    usuarios.push(professor_conhecido);

    usuarios
}
