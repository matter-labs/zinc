//!
//! The literal.
//!

pub mod boolean;
pub mod integer;
pub mod string;

use self::boolean::Literal as BooleanLiteral;
use self::integer::Literal as IntegerLiteral;
use self::string::Literal as StringLiteral;

///
/// The literal.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// The boolean literal.
    Boolean(BooleanLiteral),
    /// The integer literal.
    Integer(IntegerLiteral),
    /// The string literal.
    String(StringLiteral),
}
