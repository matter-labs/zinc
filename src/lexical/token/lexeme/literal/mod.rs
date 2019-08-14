//!
//! The literal lexeme.
//!

mod boolean;
mod integer;

pub use self::boolean::Boolean;
pub use self::integer::Integer;

use std::fmt;

use serde_derive::Serialize;

#[derive(Debug, Serialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Literal {
    Integer(Integer),
    Boolean(Boolean),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Literal::Integer(integer) => integer.to_string(),
                Literal::Boolean(boolean) => boolean.to_string(),
            }
        )
    }
}
