//!
//! The lexical token integer literal lexeme.
//!

use std::fmt;

///
/// The lexical integer literal.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Integer {
    /// A binary literal, like `0b00101010`.
    Binary {
        /// The inner literal contents.
        inner: String,
    },
    /// An octal literal, like `0o52`.
    Octal {
        /// The inner literal contents.
        inner: String,
    },
    /// An integer literal, like `42`.
    Decimal {
        /// The inner literal contents.
        inner: String,
    },
    /// A hexadecimal literal, like `0xffff`.
    Hexadecimal {
        /// The inner literal contents.
        inner: String,
    },
}

impl Integer {
    /// Characters allowed in the binary literal.
    pub const CHARACTERS_BINARY: [char; 2] = ['0', '1'];
    /// Characters allowed in the octal literal.
    pub const CHARACTERS_OCTAL: [char; 8] = ['0', '1', '2', '3', '4', '5', '6', '7'];
    /// Characters allowed in the decimal literal.
    pub const CHARACTERS_DECIMAL: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    /// Characters allowed in the hexadecimal literal.
    pub const CHARACTERS_HEXADECIMAL: [char; 22] = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B',
        'C', 'D', 'E', 'F',
    ];

    /// The zero character at the beginning of non-decimal literals.
    pub const CHARACTER_ZERO: char = '0';
    /// The binary literal second character.
    pub const CHARACTER_INITIAL_BINARY: char = 'b';
    /// The octal literal second character.
    pub const CHARACTER_INITIAL_OCTAL: char = 'o';
    /// The decimal literal second character.
    pub const CHARACTER_INITIAL_HEXADECIMAL: char = 'x';
    /// The underscore symbol which is used to separate groups of digits to improve readability.
    pub const CHARACTER_DELIMITER: char = '_';

    ///
    /// Creates a binary value.
    ///
    pub fn new_binary(inner: String) -> Self {
        Self::Binary { inner }
    }

    ///
    /// Creates an octal value.
    ///
    pub fn new_octal(inner: String) -> Self {
        Self::Octal { inner }
    }

    ///
    /// Creates a decimal value.
    ///
    pub fn new_decimal(inner: String) -> Self {
        Self::Decimal { inner }
    }

    ///
    /// Creates a hexadecimal value.
    ///
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Binary { inner } => write!(f, "{}", inner),
            Self::Octal { inner } => write!(f, "{}", inner),
            Self::Decimal { inner } => write!(f, "{}", inner),
            Self::Hexadecimal { inner } => write!(f, "{}", inner),
        }
    }
}
