//!
//! The structure expression.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;

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
