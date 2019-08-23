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
pub use self::token::Symbol;
pub use self::token::Token;
