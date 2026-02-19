use dominio::identidade::entidades::professor::Professor;
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use dominio::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use dominio::test::fabricas_de_entidades::agregados::projeto_com_coordenadores::ProjetoComCoordenadoresParcial;
use dominio::test::fabricas_de_entidades::projeto::ProjetoParcial;
use fake::Fake;
use fake::faker::boolean::pt_br::Boolean;
use uuid::Uuid;

/// Gera `qtd` projetos com coordenadores e (possíveis) vice-coordenadores aleatoriamente
/// escolhidos dentre aqueles no vetor `coordenadores`.
///
/// * Se `qtd` for 1, um é garantido que existirá 1 projeto cujo coordenador é portador do
///   o ID `id_de_coordenador_conhecido` e o tipo é `TipoDeProjeto::Extensao`.
///
/// * Se `qtd` for maior ou igual a 2, é garantido que haverá ao menos 2 projetos cujos
///   coordenadores portam o id `id_de_coordenador_conhecido`, um do tipo `Extensao` e
///   outro do tipo `IniciacaoCientifica`.
pub fn gerar_projetos(
    coordenadores: Vec<&Professor>,
    qtd: u8,
    id_de_coordenador_conhecido: Uuid,
) -> Vec<ProjetoComCoordenadores> {
    if coordenadores.is_empty() {
        return vec![];
    }

    let mut projetos = (0..qtd.saturating_sub(2))
        .map(|_| {
            let idx_coord = (0..coordenadores.len()).fake::<usize>();
            let coordenador = coordenadores[idx_coord].clone();

            let vice_coord = match Boolean(50).fake() {
                true => {
                    let mut idx_vice_coord = (0..coordenadores.len() - 1).fake::<usize>();

                    if idx_vice_coord >= idx_coord {
                        idx_vice_coord += 1;
                    };

                    Some(coordenadores[idx_vice_coord].clone())
                }
                false => None,
            };

            let tipo = match Boolean(50).fake() {
                true => TipoDeProjeto::Extensao,
                false => TipoDeProjeto::IniciacaoCientifica,
            };

            let mut projeto = ProjetoComCoordenadoresParcial::default()
                .into_builder()
                .coordenador(coordenador)
                .vice_coordenador(vice_coord)
                .build()
                .unwrap();

            projeto.obtenha_projeto_mut().coloque_tipo(tipo);

            projeto
        })
        .collect::<Vec<_>>();

    let coordenador_conhecido = coordenadores
        .iter()
        .find(|coord| *coord.obtenha_usuario().obtenha_id() == id_de_coordenador_conhecido)
        .expect("Não foi possível encontrar um coordenador com o id de coordenador conhecido");

    // Garantia de 1 Projeto de Extensao
    if qtd > 0 {
        let projeto = ProjetoParcial::default()
            .into_builder()
            .tipo(TipoDeProjeto::Extensao)
            .build()
            .unwrap();

        let projeto = ProjetoComCoordenadoresParcial::default()
            .into_builder()
            .coordenador((*coordenador_conhecido).clone())
            .projeto(projeto)
            .build()
            .unwrap();

        projetos.push(projeto);
    }

    // Garantia de 1 Projeto de Iniciação Científica
    if qtd > 1 {
        let projeto = ProjetoParcial::default()
            .into_builder()
            .tipo(TipoDeProjeto::IniciacaoCientifica)
            .build()
            .unwrap();

        let projeto = ProjetoComCoordenadoresParcial::default()
            .into_builder()
            .coordenador((*coordenador_conhecido).clone())
            .projeto(projeto)
            .build()
            .unwrap();

        projetos.push(projeto);
    }

    projetos
}
