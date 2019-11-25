//!
//! The lexical parser.
//!

mod alphabet;
mod error;
mod stream;
mod tests;
mod token;

pub use self::alphabet::Alphabet;
pub use self::error::Error;
pub use self::stream::CommentParserError;
pub use self::stream::IntegerParserError;
pub use self::stream::SymbolParserError;
pub use self::stream::TokenStream;
pub use self::stream::WordParserError;
pub use self::token::BooleanLiteral;
pub use self::token::Comment;
pub use self::token::Identifier;
pub use self::token::IdentifierError;
pub use self::token::IntegerLiteral;
pub use self::token::Keyword;
pub use self::token::KeywordError;
pub use self::token::Lexeme;
pub use self::token::Literal;
pub use self::token::Location;
pub use self::token::StringLiteral;
pub use self::token::Symbol;
pub use self::token::Token;

static PANIC_UNCHECKED_SYMBOL: &str =
    "All symbols must be checked or ruled out at this point, but this one ended up here: ";
static PANIC_UNROUTED_CHARACTER: &str =
    "All characters must be checked for being in the alphabet and routed at this point, but this one ended up here: ";
static PANIC_INTEGER_VALIDATED_DURING_SCANNING: &str = "Validity must be checked during scanning";
