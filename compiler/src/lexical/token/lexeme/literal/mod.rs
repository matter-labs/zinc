//!
//! The literal lexeme.
//!

mod boolean;
mod integer;
mod string;

pub use self::boolean::Boolean;
pub use self::integer::Integer;
pub use self::string::String;

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Void,
    Boolean(Boolean),
    Integer(Integer),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Void => write!(f, "()"),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}
