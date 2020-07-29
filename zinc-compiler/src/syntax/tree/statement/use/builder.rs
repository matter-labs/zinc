//!
//! The `use` statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::statement::r#use::Statement as UseStatement;

///
/// The `use` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The imported item path expression.
    path: Option<ExpressionTree>,
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
        )
    }
}
