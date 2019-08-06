//!
//! The literal lexeme.
//!

mod integer;

pub use self::integer::Integer;

use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub enum Literal {
    Integer(Integer),
}
