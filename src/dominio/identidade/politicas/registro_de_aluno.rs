pub fn valide_registro_de_aluno(ra: &str) -> bool {
    if ra.len() <= 1 {
        return false;
    }

    ra.starts_with('a') && ra[1..].chars().all(|char| char.is_numeric())
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::dominio::identidade::politicas::registro_de_aluno::valide_registro_de_aluno;

    #[rstest]
    #[case("a")]
    #[case("2655000")]
    #[case("a3848F3")]
    #[case("A123456")]
    #[test]
    fn deveria_rejeitar_registros_invalidos(#[case] ra: &str) {
        let registro_eh_valido = valide_registro_de_aluno(ra);
        assert!(!registro_eh_valido);
    }

    #[test]
    fn deveria_aprovar_registros_validos() {
        let registro_eh_valido = valide_registro_de_aluno("a2600321");
        assert!(registro_eh_valido);
    }
}
