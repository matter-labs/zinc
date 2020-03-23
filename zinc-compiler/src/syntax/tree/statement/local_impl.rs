//!
//! The implementation-local statement.
//!

use crate::lexical::Location;
use crate::syntax::tree::statement::r#const::Statement as ConstStatement;
use crate::syntax::tree::statement::r#fn::Statement as FnStatement;

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum Statement {
    Const(ConstStatement),
    Fn(FnStatement),
    Empty(Location),
}

impl Statement {
    pub fn location(&self) -> Location {
        match self {
            Self::Const(inner) => inner.location,
            Self::Fn(inner) => inner.location,
            Self::Empty(location) => *location,
        }
    }
}
