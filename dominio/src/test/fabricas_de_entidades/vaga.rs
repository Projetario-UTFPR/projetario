use chrono::NaiveDate;
use comum::sqlx::{DbDateTime, db_date_time_now};
use fake::faker::lorem;
use fake::{Fake, Faker};
use url::Url;
use uuid::Uuid;

use crate::projetos::agregados::projeto_com_coordenadores::ProjetoComCoordenadores;
use crate::test::fabricas_de_entidades::agregados::projeto_com_coordenadores::ProjetoComCoordenadoresParcial;
use crate::vagas::entidades::vaga::{Vaga, VagaBuilder};

#[allow(unused)]
#[derive(Default)]
pub struct VagaParcial {
    pub id: Option<Uuid>,
    pub projeto_e_coordenadores: Option<ProjetoComCoordenadores>,
    pub horas_por_semana: Option<u8>,
    pub imagem: Option<String>,
    pub quantidade: Option<u8>,
    pub link_edital: Option<String>,
    pub link_candidatura: Option<String>,
    pub titulo: Option<String>,
    pub conteudo: Option<String>,
    pub iniciada_em: Option<NaiveDate>,
    pub inscricoes_ate: Option<DbDateTime>,
    pub cancelada_em: Option<DbDateTime>,
    pub atualizada_em: Option<DbDateTime>,
}

impl VagaParcial {
    pub fn into_builder(self) -> VagaBuilder {
        let mut builder = VagaBuilder::default();

        builder
            .titulo(self.titulo)
            .conteudo(self.conteudo.unwrap_or_else(|| {
                lorem::pt_br::Paragraphs(1..4)
                    .fake::<Vec<String>>()
                    .join("\n")
            }))
            .horas_por_semana(self.horas_por_semana.unwrap_or(20))
            .id(self.id.unwrap_or_else(Uuid::new_v4))
            .imagem(self.imagem.unwrap_or_else(|| Faker.fake::<Url>().into()))
            .iniciada_em(
                self.iniciada_em
                    .unwrap_or_else(|| db_date_time_now().date()),
            )
            .inscricoes_ate(self.inscricoes_ate.unwrap_or_else(db_date_time_now))
            .cancelada_em(self.cancelada_em)
            .atualizada_em(self.atualizada_em)
            .link_candidatura(self.link_candidatura)
            .link_edital(
                self.link_edital
                    .unwrap_or_else(|| Faker.fake::<Url>().into()),
            )
            .projeto_e_coordenadores(
                self.projeto_e_coordenadores
                    .unwrap_or_else(|| ProjetoComCoordenadoresParcial::default().into_entidade()),
            )
            .quantidade(self.quantidade.unwrap_or(2));

        builder
    }

    pub fn into_entidade(self) -> Vaga { self.into_builder().build().unwrap() }
}
