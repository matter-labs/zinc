//!
//! The semantic error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::scope::error::Error as ScopeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Element(Location, ElementError),
    Scope(ScopeError),
    Expression(ExpressionError),
    Statement(StatementError),
    EntryPointMissing,
    ContractBeyondEntry,
}

impl From<ScopeError> for Error {
    fn from(inner: ScopeError) -> Self {
        Self::Scope(inner)
    }
}
