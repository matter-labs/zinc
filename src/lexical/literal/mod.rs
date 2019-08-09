//!
//! The literal lexeme.
//!

mod integer;

pub use self::integer::Integer;

use serde_derive::Serialize;

#[derive(Debug, Serialize, PartialEq)]
pub enum Literal {
    Integer(Integer),
}
