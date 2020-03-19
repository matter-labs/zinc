//!
//! The lexical token integer literal lexeme.
//!

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Integer {
    Binary { value: String },
    Octal { value: String },
    Decimal { value: String },
    Hexadecimal { value: String },
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

    pub fn new_binary(input: String) -> Self {
        Self::Binary { value: input }
    }

    pub fn new_octal(input: String) -> Self {
        Self::Octal { value: input }
    }

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
            Self::Binary { value } => value,
            Self::Octal { value } => value,
            Self::Decimal { value } => value,
            Self::Hexadecimal { value } => value,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binary { value } => write!(f, "{}", value),
            Self::Octal { value } => write!(f, "{}", value),
            Self::Decimal { value } => write!(f, "{}", value),
            Self::Hexadecimal { value } => write!(f, "{}", value),
        }
    }
}
