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

impl Integer {
    pub fn decimal(bytes: Vec<u8>) -> Self {
        Self::Decimal {
            value: String::from_utf8_lossy(&bytes).to_string(),
        }
    }

    pub fn hexadecimal(bytes: Vec<u8>) -> Self {
        Self::Hexadecimal {
            value: String::from_utf8_lossy(&bytes).to_string(),
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Decimal { value } => write!(f, "{}", value),
            Self::Hexadecimal { value } => write!(f, "{}", value),
        }
    }
}
