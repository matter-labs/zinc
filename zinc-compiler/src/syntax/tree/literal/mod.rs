//!
//! The literal.
//!

mod boolean;
mod integer;
mod string;

pub use self::boolean::Literal as BooleanLiteral;
pub use self::integer::Literal as IntegerLiteral;
pub use self::string::Literal as StringLiteral;
