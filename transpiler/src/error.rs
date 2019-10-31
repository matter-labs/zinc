//!
//! The transpiler error.
//!

use failure::Fail;

use parser::ExpressionOperator;
use parser::Location;
use semantic::CastingError;

use crate::scope::Error as ScopeError;

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "{} let declaration implicit semantic.casting: {}", _0, _1)]
    LetImplicitCasting(Location, CastingError),
    #[fail(display = "{} explicit semantic.casting: {}", _0, _1)]
    ExplicitCasting(Location, CastingError),
    #[fail(display = "{} scope: {}", _0, _1)]
    Scope(Location, ScopeError),
    #[fail(display = "{} unary operator {} expected {}", _0, _1, _2)]
    UnaryOperator(Location, ExpressionOperator, &'static str),
    #[fail(display = "{} binary operator {} expected {} and {}", _0, _1, _2, _3)]
    BinaryOperator(Location, ExpressionOperator, &'static str, &'static str),
}
