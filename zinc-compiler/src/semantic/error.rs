//!
//! The semantic error.
//!

use crate::lexical::token::location::Location;
use crate::semantic::analyzer::attribute::error::Error as AttributeError;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::scope::error::Error as ScopeError;

#[derive(Debug, PartialEq)]
pub enum Error {
    Element(ElementError),
    Scope(ScopeError),
    Expression(ExpressionError),
    Statement(StatementError),
    Attribute(AttributeError),

    EntryPointMissing,
    EntryPointAmbiguous { main: Location, contract: Location },
    EntryPointConstant { location: Location },
    FunctionMainBeyondEntry { location: Location },
    ContractBeyondEntry { location: Location },
    ModuleFileNotFound { location: Location, name: String },
}

impl From<ScopeError> for Error {
    fn from(inner: ScopeError) -> Self {
        Self::Scope(inner)
    }
}
