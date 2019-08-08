//!
//! The integer literal lexeme.
//!

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum Integer {
    Decimal(String),
    Hexadecimal(String),
}

impl From<&[u8]> for Integer {
    fn from(bytes: &[u8]) -> Self {
        if let Some(b'x') = bytes.get(1) {
            Integer::Hexadecimal(String::from_utf8_lossy(bytes).to_string())
        } else {
            Integer::Decimal(String::from_utf8_lossy(bytes).to_string())
        }
    }
}
