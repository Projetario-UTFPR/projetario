use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NullableU8(pub Option<u8>);

impl NullableU8 {
    pub fn some(value: u8) -> Self { Self(Some(value)) }
    pub fn none() -> Self { Self(None) }

    pub fn into_inner(self) -> Option<u8> { self.0 }

    pub fn to_optional<T: From<u8>>(&self) -> Option<T> { Some(T::from(self.0?)) }
}

impl Deref for NullableU8 {
    type Target = Option<u8>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

macro_rules! define_from {
    ($type:ty) => {
        impl From<Option<$type>> for NullableU8 {
            fn from(value: Option<$type>) -> Self {
                match value {
                    None => Self(None),
                    Some(value) => {
                        #[allow(unused_comparisons)]
                        if value < 0 || value > u8::MAX as $type {
                            panic!(
                                "Tentou converter um valor fora do intervalo \
                                permitido em um u8 em um NullableU8"
                            );
                        }

                        Self(Some(value as u8))
                    }
                }
            }
        }
    };
}

define_from!(i8);
define_from!(i16);
define_from!(i32);
define_from!(i64);
define_from!(u8);
define_from!(u16);
define_from!(u32);
