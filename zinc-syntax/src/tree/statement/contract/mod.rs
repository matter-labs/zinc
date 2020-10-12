//!
//! The `contract` statement.
//!

pub mod builder;

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::statement::local_contract::Statement as ContractLocalStatement;

///
/// The `contract` statement.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The location of the syntax construction.
    pub location: Location,
    /// The contract type identifier.
    pub identifier: Identifier,
    /// The contract statements.
    pub statements: Vec<ContractLocalStatement>,
}

impl Statement {
    ///
    /// Creates a `contract` statement.
    ///
    pub fn new(
        location: Location,
        identifier: Identifier,
        statements: Vec<ContractLocalStatement>,
    ) -> Self {
        Self {
            location,
            identifier,
            statements,
        }
    }
}
