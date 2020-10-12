//!
//! The function argument list expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::list::Expression as ListExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;

///
/// The function argument list expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The function argument list inner expressions.
    expressions: Vec<ExpressionTree>,
}

impl Builder {
    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    ///
    /// Pushes the corresponding builder value.
    ///
    pub fn push_expression(&mut self, value: ExpressionTree) {
        self.expressions.push(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> ListExpression {
        ListExpression::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.expressions,
        )
    }
}
