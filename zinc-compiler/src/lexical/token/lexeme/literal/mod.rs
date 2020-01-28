//!
//! The lexical token literal lexeme.
//!

pub mod boolean;
pub mod integer;
pub mod string;

use std::fmt;

use self::boolean::Boolean;
use self::integer::Integer;
use self::string::String;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Boolean(Boolean),
    Integer(Integer),
    String(String),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::Integer(integer) => write!(f, "{}", integer),
            Self::String(string) => write!(f, "{}", string),
        }
    }
}
