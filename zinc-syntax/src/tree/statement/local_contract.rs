//!
//! The contract-local statement.
//!

use zinc_lexical::Location;

use crate::tree::statement::field::Statement as FieldStatement;
use crate::tree::statement::r#const::Statement as ConstStatement;
use crate::tree::statement::r#fn::Statement as FnStatement;

///
/// The contract-level statement.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// The `field` statement.
    Field(FieldStatement),
    /// The `const` statement.
    Const(ConstStatement),
    /// The `fn` statement.
    Fn(FnStatement),
    /// The empty `;` statement.
    Empty(Location),
}

impl Statement {
    ///
    /// The statement location.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::Field(inner) => inner.location,
            Self::Const(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Empty(location) => *location,
        }
    }
}
