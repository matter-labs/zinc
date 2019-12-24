//!
//! The lexical token integer literal lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Integer {
    Decimal { value: String },
    Hexadecimal { value: String },
}

impl Integer {
    pub fn new_decimal(input: String) -> Self {
        Self::Decimal { value: input }
    }

    pub fn new_hexadecimal(input: String) -> Self {
        Self::Hexadecimal { value: input }
    }
}

impl Into<String> for Integer {
    fn into(self) -> String {
        match self {
            Self::Decimal { value } => value,
            Self::Hexadecimal { value } => value,
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
