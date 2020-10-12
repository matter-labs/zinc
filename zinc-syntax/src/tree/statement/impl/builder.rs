//!
//! The `impl` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::identifier::Identifier;
use crate::tree::statement::local_impl::Statement as ImplementationLocalStatement;
use crate::tree::statement::r#impl::Statement as ImplStatement;

///
/// The `impl` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The identifier of the implemented type.
    identifier: Option<Identifier>,
    /// The implementation statements.
    statements: Vec<ImplementationLocalStatement>,
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
    pub fn push_statement(&mut self, statement: ImplementationLocalStatement) {
        self.statements.push(statement);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ImplStatement {
        ImplStatement::new(
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
