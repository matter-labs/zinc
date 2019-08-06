//!
//! The literal lexeme.
//!

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum Integer {
    Decimal,
    Hexadecimal,
}

impl From<&[u8]> for Integer {
    fn from(bytes: &[u8]) -> Self {
        if bytes[1] == b'x' {
            Integer::Hexadecimal
        } else {
            Integer::Decimal
        }
    }
}
