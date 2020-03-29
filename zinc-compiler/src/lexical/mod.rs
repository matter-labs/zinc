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

pub mod error;
pub mod stream;
pub mod tests;
pub mod token;

pub use self::error::Error;
pub use self::stream::TokenStream;
pub use self::token::lexeme::identifier::Identifier;
pub use self::token::lexeme::keyword::Keyword;
pub use self::token::lexeme::literal::boolean::Boolean as BooleanLiteral;
pub use self::token::lexeme::literal::integer::Integer as IntegerLiteral;
pub use self::token::lexeme::literal::string::String as StringLiteral;
pub use self::token::lexeme::literal::Literal;
pub use self::token::lexeme::symbol::Symbol;
pub use self::token::lexeme::Lexeme;
pub use self::token::location::Location;
pub use self::token::Token;
