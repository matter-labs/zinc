//!
//! The `use` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;
use crate::tree::statement::r#use::Statement as UseStatement;

///
/// The `use` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The imported item path expression.
    path: Option<ExpressionTree>,
    /// The imported item optional alias.
    alias_identifier: Option<Identifier>,
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
    pub fn set_path(&mut self, value: ExpressionTree) {
        self.path = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_alias_identifier(&mut self, value: Identifier) {
        self.alias_identifier = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> UseStatement {
        UseStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.path.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "path")
            }),
            self.alias_identifier.take(),
        )
    }
}
