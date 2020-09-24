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
        /// The pseudo-integer part.
        integer: String,
        /// The optional pseudo-fractional part.
        fractional: Option<String>,
        /// The optional pseudo-exponent part.
        exponent: Option<String>,
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
    /// The decimal point which is used to separate pseudo-integer and -fractional parts.
    pub const CHARACTER_DECIMAL_POINT: char = '.';
    /// The exponent character which specifies how many zeros must be added to the pseudo-fractional value.
    pub const CHARACTER_EXPONENT: char = 'E';

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
    pub fn new_decimal(integer: String) -> Self {
        Self::Decimal {
            integer,
            fractional: None,
            exponent: None,
        }
    }

    ///
    /// Creates a decimal value.
    ///
    pub fn new_decimal_with_exponent(
        integer: String,
        fractional: Option<String>,
        exponent: Option<String>,
    ) -> Self {
        Self::Decimal {
            integer,
            fractional,
            exponent,
        }
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
            Self::Decimal {
                integer,
                fractional,
                exponent,
            } => format!(
                "{}.{}E{}",
                integer,
                fractional.unwrap_or_default(),
                exponent.unwrap_or_default()
            ),
            Self::Hexadecimal { inner } => inner,
        }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.to_owned().into();
        write!(f, "{}", string)
    }
}
