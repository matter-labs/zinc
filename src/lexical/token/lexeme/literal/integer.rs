//!
//! The integer literal lexeme.
//!

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Integer {
    Decimal { value: String },
    Hexadecimal { value: String },
}

impl From<&[u8]> for Integer {
    fn from(bytes: &[u8]) -> Self {
        if let Some(b"0x") = bytes.get(0..2) {
            Integer::Hexadecimal {
                value: String::from_utf8_lossy(&bytes[2..]).to_string(),
            }
        } else {
            Integer::Decimal {
                value: String::from_utf8_lossy(bytes).to_string(),
            }
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Integer::Decimal { value } => value,
                Integer::Hexadecimal { value } => value,
            }
        )
    }
}
