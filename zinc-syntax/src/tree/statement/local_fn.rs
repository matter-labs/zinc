//!
//! The function-local statement.
//!

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::statement::r#const::Statement as ConstStatement;
use crate::tree::statement::r#for::Statement as ForStatement;
use crate::tree::statement::r#let::Statement as LetStatement;

///
/// The function-or-block-level statement.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// The `let` statement.
    Let(LetStatement),
    /// The `const` statement.
    Const(ConstStatement),
    /// The `for` statement.
    For(ForStatement),
    /// The empty `;` statement.
    Empty(Location),
    /// The expression statement.
    Expression(ExpressionTree),
}

impl Statement {
    ///
    /// The statement location.
    ///
    pub fn location(&self) -> Location {
        match self {
            Self::Let(inner) => inner.location,
            Self::Const(inner) => inner.location,
            Self::For(inner) => inner.location,
            Self::Empty(location) => *location,
            Self::Expression(inner) => inner.location,
        }
    }
}
