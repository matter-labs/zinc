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
        if let Some(b'x') = bytes.get(1) {
            Integer::Hexadecimal
        } else {
            Integer::Decimal
        }
    }
}
