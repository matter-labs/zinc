//!
//! The implementation-local statement.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

///
/// The implementation-level statement.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
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
            Self::Const(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Empty(location) => *location,
        }
    }
}
