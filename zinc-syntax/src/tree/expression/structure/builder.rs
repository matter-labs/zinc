//!
//! The structure expression builder.
//!

use zinc_lexical::Location;

use crate::tree::expression::structure::Expression as StructureExpression;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::Identifier;

///
/// The structure expression builder.
///
#[derive(Default)]
pub struct Builder {
    /// The location of the syntax construction.
    location: Option<Location>,
    /// The structure expression inner fields.
    fields: Vec<(Identifier, Option<ExpressionTree>)>,
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
    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    ///
    /// Sets the corresponding builder value.
    ///
    pub fn set_field_expression(&mut self, value: ExpressionTree) {
        self.fields
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "field identifier"
                )
            })
            .1 = Some(value);
    }

    ///
    /// Finalizes the builder and returns the built value.
    ///
    /// # Panics
    /// If some of the required items has not been set.
    ///
    pub fn finish(self) -> StructureExpression {
        StructureExpression::new(
            self.location.unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    zinc_const::panic::BUILDER_REQUIRES_VALUE,
                    "location"
                )
            }),
            self.fields
                .into_iter()
                .map(|(identifier, expression)| {
                    (
                        identifier,
                        expression.unwrap_or_else(|| {
                            panic!(
                                "{}{}",
                                zinc_const::panic::BUILDER_REQUIRES_VALUE,
                                "field expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(Identifier, ExpressionTree)>>(),
        )
    }
}
