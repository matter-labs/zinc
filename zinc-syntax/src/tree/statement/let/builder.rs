//!
//! The `let` statement builder.
//!

use zinc_lexical::Location;

use crate::tree::binding::Binding;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::statement::r#let::Statement as LetStatement;

///
/// The `let` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The binding.
    binding: Option<Binding>,
    /// The expression assigned to the variable.
    expression: Option<ExpressionTree>,
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
    pub fn set_binding(&mut self, value: Binding) {
        self.binding = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_expression(&mut self, value: ExpressionTree) {
        self.expression = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(mut self) -> LetStatement {
        LetStatement::new(
            self.location.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.binding.take().unwrap_or_else(|| {
                panic!("{}{}", zinc_const::panic::BUILDER_REQUIRES_VALUE, "binding")
            }),
            self.expression.take().unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "expression"
                )
            }),
        )
    }
}
