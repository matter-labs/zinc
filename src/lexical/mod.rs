//!
//! The lexical analysis.
//!

mod alphabet;
mod comment;
mod delimiter;
mod error;
mod identifier;
mod keyword;
mod literal;
mod operator;
mod punctuation;
mod stream;
mod token;

pub use self::alphabet::Alphabet;
pub use self::comment::Comment;
pub use self::delimiter::Delimiter;
pub use self::error::Error;
pub use self::identifier::Error as IdentifierError;
pub use self::identifier::Identifier;
pub use self::keyword::Error as KeywordError;
pub use self::keyword::Keyword;
pub use self::literal::Integer as IntegerLiteral;
pub use self::literal::Literal;
pub use self::operator::Error as OperatorError;
pub use self::operator::Operator;
pub use self::punctuation::Punctuation;
pub use self::stream::CommentAnalyzerError;
pub use self::stream::IntegerAnalyzerError;
pub use self::stream::OperatorAnalyzerError;
pub use self::stream::Stream;
pub use self::token::Lexeme;
pub use self::token::Location;
pub use self::token::Token;
