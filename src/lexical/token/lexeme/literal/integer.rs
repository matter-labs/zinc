//!
//! The integer literal lexeme.
//!

use std::fmt;
use std::str;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum Integer {
    Decimal { value: Vec<u8> },
    Hexadecimal { value: Vec<u8> },
}

impl Integer {
    pub fn decimal(bytes: Vec<u8>) -> Self {
        Self::Decimal { value: bytes }
    }

    pub fn hexadecimal(bytes: Vec<u8>) -> Self {
        Self::Hexadecimal { value: bytes }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Decimal { value } => write!(f, "{}", unsafe { str::from_utf8_unchecked(&value) }),
            Self::Hexadecimal { value } => {
                write!(f, "{}", unsafe { str::from_utf8_unchecked(&value) })
            }
        }
    }
}
