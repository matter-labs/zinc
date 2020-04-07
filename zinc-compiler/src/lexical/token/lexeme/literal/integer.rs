//!
//! The lexical token integer literal lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Integer {
    Binary { inner: String },
    Octal { inner: String },
    Decimal { inner: String },
    Hexadecimal { inner: String },
}

impl Integer {
    pub const CHARACTERS_BINARY: [char; 2] = ['0', '1'];
    pub const CHARACTERS_OCTAL: [char; 8] = ['0', '1', '2', '3', '4', '5', '6', '7'];
    pub const CHARACTERS_DECIMAL: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    pub const CHARACTERS_HEXADECIMAL: [char; 22] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B',
        'C', 'D', 'E', 'F',
    ];

    pub const CHARACTER_ZERO: char = '0';
    pub const CHARACTER_INITIAL_BINARY: char = 'b';
    pub const CHARACTER_INITIAL_OCTAL: char = 'o';
    pub const CHARACTER_INITIAL_HEXADECIMAL: char = 'x';
    pub const CHARACTER_DELIMITER: char = '_';

    pub fn new_binary(inner: String) -> Self {
        Self::Binary { inner }
    }

    pub fn new_octal(inner: String) -> Self {
        Self::Octal { inner }
    }

    pub fn new_decimal(inner: String) -> Self {
        Self::Decimal { inner }
    }

    pub fn new_hexadecimal(inner: String) -> Self {
        Self::Hexadecimal { inner }
    }
}

impl Into<String> for Integer {
    fn into(self) -> String {
        match self {
            Self::Binary { inner } => inner,
            Self::Octal { inner } => inner,
            Self::Decimal { inner } => inner,
            Self::Hexadecimal { inner } => inner,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binary { inner } => write!(f, "{}", inner),
            Self::Octal { inner } => write!(f, "{}", inner),
            Self::Decimal { inner } => write!(f, "{}", inner),
            Self::Hexadecimal { inner } => write!(f, "{}", inner),
        }
    }
}
