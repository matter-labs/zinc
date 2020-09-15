//!
//! The lexical token keyword lexeme.
//!

use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
use std::str;

///
/// The keyword defined in the language.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    /// The `let` declaration keyword.
    Let,
    /// The `mut` declaration keyword.
    Mut,
    /// The `const` declaration keyword.
    Const,
    /// The `type` declaration keyword.
    Type,
    /// The `struct` declaration keyword.
    Struct,
    /// The `enum` declaration keyword.
    Enum,
    /// The `fn` declaration keyword.
    Fn,
    /// The `mod` declaration keyword.
    Mod,
    /// The `use` declaration keyword.
    Use,
    /// The `impl` declaration keyword.
    Impl,
    /// The `contract` declaration keyword.
    Contract,
    /// The `pub` declaration keyword.
    Pub,

    /// The `for` control keyword.
    For,
    /// The `in` control keyword.
    In,
    /// The `while` control keyword.
    While,
    /// The `if` control keyword.
    If,
    /// The `else` control keyword.
    Else,
    /// The `match` control keyword.
    Match,

    /// The `bool` type keyword.
    Bool,
    /// The `u{N}` type keyword.
    IntegerUnsigned {
        /// The unsigned type bitlength.
        bitlength: usize,
    },
    /// The `i{N}` type keyword.
    IntegerSigned {
        /// The signed type bitlength.
        bitlength: usize,
    },
    /// The `field` type keyword.
    Field,

    /// The `true` literal keyword.
    True,
    /// The `false` literal keyword.
    False,

    /// The `as` operator keyword.
    As,

    /// The `crate` alias keyword.
    Crate,
    /// The `super` alias keyword.
    Super,
    /// The `self` alias keyword.
    SelfLowercase,
    /// The `Self` alias keyword.
    SelfUppercase,

    /// The `static` reserved keyword.
    Static,
    /// The `ref` reserved keyword.
    Ref,
    /// The `extern` reserved keyword.
    Extern,
    /// The `return` reserved keyword.
    Return,
    /// The `loop` reserved keyword.
    Loop,
    /// The `break` reserved keyword.
    Break,
    /// The `continue` reserved keyword.
    Continue,
    /// The `trait` reserved keyword.
    Trait,
}

impl Keyword {
    /// The range including the minimal and maximal integer bitlengths.
    pub const INTEGER_BITLENGTH_RANGE: RangeInclusive<usize> =
        zinc_const::bitlength::BYTE..=zinc_const::bitlength::INTEGER_MAX;

    ///
    /// Creates a `u{N}` keyword.
    ///
    pub fn new_integer_unsigned(bitlength: usize) -> Self {
        Self::IntegerUnsigned { bitlength }
    }

    ///
    /// Creates an `i{N}` keyword.
    ///
    pub fn new_integer_signed(bitlength: usize) -> Self {
        Self::IntegerSigned { bitlength }
    }

    ///
    /// Checks if the keyword is an alias.
    ///
    pub fn is_alias(name: &str) -> bool {
        name == Self::Crate.to_string().as_str()
            || name == Self::Super.to_string().as_str()
            || name == Self::SelfLowercase.to_string().as_str()
            || name == Self::SelfUppercase.to_string().as_str()
    }
}

///
/// The keyword parsing error.
///
/// If the parser returns such an error, it means that the word is not a keyword,
/// but an ordinar identifier or something else.
///
#[derive(Debug)]
pub enum Error {
    /// There is no number after the `u` or `i` character.
    IntegerBitlengthEmpty,
    /// There is an invalid after the `u` or `i` character.
    IntegerBitlengthNotNumeric(String),
    /// The bitlength is not multiple of `8`, which is forbidden.
    IntegerBitlengthNotMultipleOfEight(usize, usize),
    /// The bitlength is beyond the allowed range.
    IntegerBitlengthOutOfRange(usize, RangeInclusive<usize>),
    /// The keyword is unknown, which means that the word is a valid identifier or something else.
    Unknown(String),
}

impl TryFrom<&str> for Keyword {
    type Error = Error;

    ///
    /// The converter checks if the number after the `u` or `i` symbol represents a valid
    /// amount of bits for an integer value. If the amount is not a valid bitlength, the word is
    /// treated as an ordinar identifier.
    ///
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "let" => return Ok(Self::Let),
            "mut" => return Ok(Self::Mut),
            "const" => return Ok(Self::Const),
            "type" => return Ok(Self::Type),
            "struct" => return Ok(Self::Struct),
            "enum" => return Ok(Self::Enum),
            "fn" => return Ok(Self::Fn),
            "mod" => return Ok(Self::Mod),
            "use" => return Ok(Self::Use),
            "impl" => return Ok(Self::Impl),
            "contract" => return Ok(Self::Contract),
            "pub" => return Ok(Self::Pub),

            "for" => return Ok(Self::For),
            "in" => return Ok(Self::In),
            "while" => return Ok(Self::While),
            "if" => return Ok(Self::If),
            "else" => return Ok(Self::Else),
            "match" => return Ok(Self::Match),

            "bool" => return Ok(Self::Bool),
            "field" => return Ok(Self::Field),

            "true" => return Ok(Self::True),
            "false" => return Ok(Self::False),

            "as" => return Ok(Self::As),

            "crate" => return Ok(Self::Crate),
            "super" => return Ok(Self::Super),
            "self" => return Ok(Self::SelfLowercase),
            "Self" => return Ok(Self::SelfUppercase),

            "static" => return Ok(Self::Static),
            "ref" => return Ok(Self::Ref),
            "extern" => return Ok(Self::Extern),
            "return" => return Ok(Self::Return),
            "loop" => return Ok(Self::Loop),
            "break" => return Ok(Self::Break),
            "continue" => return Ok(Self::Continue),
            "trait" => return Ok(Self::Trait),

            _ => {}
        }

        if let Some("u") = input.get(..1) {
            let bitlength = &input[1..];
            if bitlength.is_empty() {
                return Err(Error::IntegerBitlengthEmpty);
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::IntegerBitlengthNotNumeric(bitlength.to_owned()))?;
            if !Self::INTEGER_BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    Self::INTEGER_BITLENGTH_RANGE,
                ));
            }
            if bitlength % zinc_const::bitlength::BYTE != 0 {
                return Err(Error::IntegerBitlengthNotMultipleOfEight(
                    bitlength,
                    zinc_const::bitlength::BYTE,
                ));
            }
            return Ok(Self::new_integer_unsigned(bitlength));
        }

        if let Some("i") = input.get(..1) {
            let bitlength = &input[1..];
            if bitlength.is_empty() {
                return Err(Error::IntegerBitlengthEmpty);
            }
            let bitlength = bitlength
                .parse::<usize>()
                .map_err(|_| Error::IntegerBitlengthNotNumeric(bitlength.to_owned()))?;
            if !Self::INTEGER_BITLENGTH_RANGE.contains(&bitlength) {
                return Err(Error::IntegerBitlengthOutOfRange(
                    bitlength,
                    Self::INTEGER_BITLENGTH_RANGE,
                ));
            }
            if bitlength % zinc_const::bitlength::BYTE != 0 {
                return Err(Error::IntegerBitlengthNotMultipleOfEight(
                    bitlength,
                    zinc_const::bitlength::BYTE,
                ));
            }
            return Ok(Self::new_integer_signed(bitlength));
        }

        Err(Error::Unknown(input.to_owned()))
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let => write!(f, "let"),
            Self::Mut => write!(f, "mut"),
            Self::Const => write!(f, "const"),
            Self::Type => write!(f, "type"),
            Self::Struct => write!(f, "struct"),
            Self::Enum => write!(f, "enum"),
            Self::Fn => write!(f, "fn"),
            Self::Mod => write!(f, "mod"),
            Self::Use => write!(f, "use"),
            Self::Impl => write!(f, "impl"),
            Self::Contract => write!(f, "contract"),
            Self::Pub => write!(f, "pub"),

            Self::For => write!(f, "for"),
            Self::In => write!(f, "in"),
            Self::While => write!(f, "while"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Match => write!(f, "match"),

            Self::Bool => write!(f, "bool"),
            Self::IntegerUnsigned { bitlength } => write!(f, "u{}", bitlength),
            Self::IntegerSigned { bitlength } => write!(f, "i{}", bitlength),
            Self::Field => write!(f, "field"),

            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),

            Self::As => write!(f, "as"),

            Self::Crate => write!(f, "crate"),
            Self::Super => write!(f, "super"),
            Self::SelfLowercase => write!(f, "self"),
            Self::SelfUppercase => write!(f, "Self"),

            Self::Static => write!(f, "static"),
            Self::Ref => write!(f, "ref"),
            Self::Extern => write!(f, "extern"),
            Self::Return => write!(f, "return"),
            Self::Loop => write!(f, "loop"),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::Trait => write!(f, "trait"),
        }
    }
}
