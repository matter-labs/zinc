//!
//! The `let` statement builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;
use crate::syntax::tree::r#type::Type;
use crate::syntax::tree::statement::r#let::Statement as LetStatement;

///
/// The `let` statement builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The variable identifier.
    identifier: Option<Identifier>,
    /// If the variable is mutable.
    is_mutable: bool,
    /// The optional variable type, which is inferred otherwise.
    r#type: Option<Type>,
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
    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_mutable(&mut self) {
        self.is_mutable = true;
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_type(&mut self, value: Type) {
        self.r#type = Some(value);
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
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.is_mutable,
            self.r#type.take(),
            self.expression.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "expression")
            }),
        )
    }
}
