//! uint_n traits
//! They are implemented in the uint_n crate as proc macros.

pub trait Field {
    fn to_bytes_le(&self) -> Vec<u8>;
}
