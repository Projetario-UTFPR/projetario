use comum::sqlx::DbDateTime;
use proptest::prelude::*;
use proptest_arbitrary_interop::arb;
use uuid::Uuid;

use crate::comum::filtragem::LimitadorDeData;
use crate::projetos::enums::tipo_de_projeto::TipoDeProjeto;
use crate::vagas::filtragem::EstadoDaVaga;

pub fn arb_uuid() -> impl Strategy<Value = Uuid> { any::<[u8; 16]>().prop_map(Uuid::from_bytes) }

pub fn arb_tipo() -> impl Strategy<Value = TipoDeProjeto> {
    prop_oneof![
        Just(TipoDeProjeto::Extensao),
        Just(TipoDeProjeto::IniciacaoCientifica),
    ]
}

pub fn arb_estado() -> impl Strategy<Value = EstadoDaVaga> {
    prop_oneof![Just(EstadoDaVaga::Ativa), Just(EstadoDaVaga::Encerrada),]
}

pub fn arb_limitador() -> impl Strategy<Value = LimitadorDeData> {
    prop_oneof![Just(LimitadorDeData::Ate), Just(LimitadorDeData::Apos),]
}

pub fn arb_db_date_time() -> impl Strategy<Value = DbDateTime> { arb::<DbDateTime>() }
