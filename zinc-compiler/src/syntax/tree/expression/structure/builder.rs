//!
//! The structure or identifier expression builder.
//!

use crate::lexical::token::location::Location;
use crate::syntax::tree::expression::structure::Expression as StructureExpression;
use crate::syntax::tree::expression::tree::Tree as ExpressionTree;
use crate::syntax::tree::identifier::Identifier;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    is_structure: bool,
    fields: Vec<(Identifier, Option<ExpressionTree>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_is_structure(&mut self) {
        self.is_structure = true;
    }

    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    pub fn set_field_expression(&mut self, value: ExpressionTree) {
        self.fields
            .last_mut()
            .unwrap_or_else(|| {
                panic!(
                    "{}{}",
                    crate::panic::BUILDER_REQUIRES_VALUE,
                    "field identifier"
                )
            })
            .1 = Some(value);
    }

    pub fn finish(mut self) -> StructureExpression {
        StructureExpression::new(
            self.location.unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "location")
            }),
            self.identifier.take().unwrap_or_else(|| {
                panic!("{}{}", crate::panic::BUILDER_REQUIRES_VALUE, "identifier")
            }),
            self.is_structure,
            self.fields
                .into_iter()
                .map(|(identifier, expression)| {
                    (
                        identifier,
                        expression.unwrap_or_else(|| {
                            panic!(
                                "{}{}",
                                crate::panic::BUILDER_REQUIRES_VALUE,
                                "field expression"
                            )
                        }),
                    )
                })
                .collect::<Vec<(Identifier, ExpressionTree)>>(),
        )
    }
}
