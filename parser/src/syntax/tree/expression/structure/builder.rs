//!
//! The structure expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Expression;
use crate::syntax::ExpressionBuilder;
use crate::syntax::ExpressionOperand;
use crate::syntax::Identifier;
use crate::syntax::StructureExpression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    identifier: Option<Identifier>,
    has_bracket: bool,
    fields: Vec<(Identifier, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_identifier(&mut self, value: Identifier) {
        self.identifier = Some(value);
    }

    pub fn set_bracket(&mut self) {
        self.has_bracket = true;
    }

    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    pub fn set_field_expression(&mut self, value: Expression) {
        self.fields.last_mut().expect("Missing field identifier").1 = Some(value);
    }

    pub fn finish(mut self) -> Expression {
        match (self.fields.len(), self.has_bracket) {
            (0, false) => {
                let mut builder = ExpressionBuilder::default();
                let location = self.location.take().expect("Missing location");
                builder.set_location(location);
                builder.push_operand(
                    location,
                    ExpressionOperand::Identifier(
                        self.identifier.take().expect("Missing identifier"),
                    ),
                );
                builder.finish()
            }
            (_size, true) => {
                let mut builder = ExpressionBuilder::default();
                let location = self.location.take().expect("Missing location");
                builder.set_location(location);
                builder.push_operand(
                    location,
                    ExpressionOperand::Structure(StructureExpression::new(
                        location,
                        self.identifier.take().expect("Missing identifier"),
                        self.fields
                            .into_iter()
                            .map(|(identifier, expression)| {
                                (identifier, expression.expect("Missing field expression"))
                            })
                            .collect::<Vec<(Identifier, Expression)>>(),
                    )),
                );
                builder.finish()
            }
            _ => panic!("Always checked by the branches above"),
        }
    }
}
