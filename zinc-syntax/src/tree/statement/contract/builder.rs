//!
//! The `contract` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::statement::contract::Statement as ContractStatement;
use crate::tree::statement::local_contract::Statement as ContractLocalStatement;

///
/// The `contract` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The contract type identifier.
    identifier: Option<Identifier>,
    /// The contract statements.
    statements: Vec<ContractLocalStatement>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_statement(&mut self, statement: ContractLocalStatement) {
        self.statements.push(statement);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ContractStatement {
        ContractStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "identifier"
                )
            }),
            self.statements,
        )
    }
}
