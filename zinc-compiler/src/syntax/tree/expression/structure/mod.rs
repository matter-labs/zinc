//!
//! The structure expression.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

///
/// The structure expression.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    /// The location of the syntax construction.
    pub location: Location,
    /// The structure expression inner fields.
    pub fields: Vec<(Identifier, ExpressionTree)>,
}

impl Expression {
    ///
    /// Creates a structure expression.
    ///
    pub fn new(location: Location, fields: Vec<(Identifier, ExpressionTree)>) -> Self {
        Self { location, fields }
    }
}
