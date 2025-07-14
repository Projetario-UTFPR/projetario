pub struct NullableU8(pub Option<u8>);

impl NullableU8 {
    pub fn some(value: u8) -> Self { Self(Some(value)) }
    pub fn none() -> Self { Self(None) }

    pub fn into_inner(self) -> Option<u8> { self.0 }

    pub fn to_optional_i32(&self) -> Option<i32> { Some(self.0? as i32) }
}

impl From<Option<i32>> for NullableU8 {
    fn from(value: Option<i32>) -> Self {
        match value {
            None => Self(None),
            Some(value) => {
                if value < 0 || value > u8::MAX as i32 {
                    panic!(
                        "Tentou converter um valor fora do intervalo permitido em um u8 em um NullableU8"
                    );
                }

                Self(Some(value as u8))
            }
        }
    }
}
