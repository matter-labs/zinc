//!
//! The lexical parser.
//!
//! \t \n \r
//! <Space> "
//! % &
//! ( ) * + , - . / 0 1 2 3 4 5 6 7 8 9 : ; < = >
//! A B C D E F G H I J K L M N O P Q R S T U V W X Y Z [ \ ] ^ _
//! a b c d e f g h i j k l m n o p q r s t u v w x y z { | }
//!

mod error;
mod stream;
mod tests;
mod token;

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
