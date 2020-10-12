//!
//! The `impl` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::statement::local_impl::Statement as ImplementationLocalStatement;

///
/// The `impl` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The identifier of the implemented type.
    pub identifier: Identifier,
    /// The implementation statements.
    pub statements: Vec<ImplementationLocalStatement>,
}

impl Statement {
    ///
    /// Creates an `impl` statement.
    ///
    pub fn new(
        location: Location,
        identifier: Identifier,
        statements: Vec<ImplementationLocalStatement>,
    ) -> Self {
        Self {
            location,
            identifier,
            statements,
        }
    }
}
