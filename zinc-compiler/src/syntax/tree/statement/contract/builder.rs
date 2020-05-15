//!
//! The contract statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::statement::contract::Statement as ContractStatement;
use crate::syntax::tree::statement::local_contract::Statement as ContractLocalStatement;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    statements: Vec<ContractLocalStatement>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn push_statement(&mut self, statement: ContractLocalStatement) {
        self.statements.push(statement);
    }

    pub fn finish(mut self) -> ContractStatement {
        ContractStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.statements,
        )
    }
}
