//!
//! The contract statement.
//!

pub mod builder;

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;

#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub location: Location,
    pub identifier: Identifier,
    pub statements: Vec<ContractLocalStatement>,
}

impl Statement {
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
