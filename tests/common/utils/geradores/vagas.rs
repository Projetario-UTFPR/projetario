use chrono::{DateTime, Duration, NaiveDateTime, Timelike, Utc};
use dominio::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use dominio::test::fabricas_de_entidades::vaga::VagaParcial;
use dominio::vagas::entidades::vaga::Vaga;
use fake::Fake;
use fake::faker::boolean::pt_br::Boolean;
use fake::faker::chrono::pt_br::{DateTimeAfter, DateTimeBefore};

pub fn gerar_vagas_de_projetos(
    projetos: Vec<&ProjetoComCoordenadores>,
    ativadas: u8,
    concluidas: u8,
    canceladas: u8,
    titulo_conhecido: String,
    data_conhecida: NaiveDateTime,
) -> Vec<Vaga> {
    let mut vagas = Vec::new();

    let talvez_concatene_titulo_conhecido = |vaga: &mut Vaga, p: u8| {
        if Boolean(p).fake() {
            vaga.coloque_titulo(format!("{} {}", vaga.obtenha_titulo(), titulo_conhecido));
        }
    };

    for i in 0..ativadas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();
        let data_limite = DateTimeAfter(data_conhecida.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc()
            .with_nanosecond(0)
            .unwrap();

        let data_de_inicio = if i % 5 == 0 {
            let data_posterior = DateTimeAfter(data_conhecida.and_utc())
                .fake::<DateTime<Utc>>()
                .naive_utc();

            Some(data_posterior + Duration::days(1))
        } else if i % 3 == 0 {
            let data_anterior = DateTimeBefore(data_conhecida.and_utc())
                .fake::<DateTime<Utc>>()
                .naive_utc();

            Some(data_anterior - Duration::days(1))
        } else {
            None
        };

        let mut vaga_builder = VagaParcial::default().into_builder();

        vaga_builder
            .projeto_e_coordenadores(projeto)
            .cancelada_em(None)
            .inscricoes_ate(data_limite);

        if let Some(data) = data_de_inicio {
            vaga_builder.iniciada_em(data);
        }

        let mut vaga = vaga_builder.build().unwrap();

        if i == ativadas.saturating_sub(1) {
            talvez_concatene_titulo_conhecido(&mut vaga, 100);
        } else {
            talvez_concatene_titulo_conhecido(&mut vaga, 50);
        }

        vagas.push(vaga);
    }

    for _ in 0..concluidas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();
        let data_limite = DateTimeBefore(data_conhecida.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc();

        let mut vaga = VagaParcial::default()
            .into_builder()
            .projeto_e_coordenadores(projeto)
            .cancelada_em(None)
            .inscricoes_ate(data_limite)
            .build()
            .unwrap();

        talvez_concatene_titulo_conhecido(&mut vaga, 50);

        vagas.push(vaga);
    }

    for _ in 0..canceladas {
        let projeto = projetos[(0..projetos.len()).fake::<usize>()].clone();

        let data_limite = DateTimeAfter(data_conhecida.and_utc())
            .fake::<DateTime<Utc>>()
            .naive_utc()
            .with_nanosecond(0)
            .unwrap();

        let data_cancelamento = fake::faker::chrono::pt_br::DateTime()
            .fake::<DateTime<Utc>>()
            .naive_utc();

        let mut vaga = VagaParcial::default()
            .into_builder()
            .projeto_e_coordenadores(projeto)
            .cancelada_em(data_cancelamento)
            .inscricoes_ate(data_limite)
            .build()
            .unwrap();

        talvez_concatene_titulo_conhecido(&mut vaga, 50);

        vagas.push(vaga);
    }

    vagas
}
