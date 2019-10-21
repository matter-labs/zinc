//!
//! The structure expression builder.
//!

use crate::lexical::Location;
use crate::syntax::Identifier;
use crate::syntax::StructureExpression;
use crate::syntax::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    path: Option<Expression>,
    fields: Vec<(Identifier, Option<Expression>)>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_path(&mut self, value: Expression) {
        self.path = Some(value);
    }

    pub fn push_field_identifier(&mut self, value: Identifier) {
        self.fields.push((value, None));
    }

    pub fn set_field_expression(&mut self, value: Expression) {
        self.fields.last_mut().expect("Missing field identifier").1 = Some(value);
    }

    pub fn finish(mut self) -> StructureExpression {
        StructureExpression::new(
                self.location.expect("Missing location"),
                self.path.take().expect("Missing path"),
                self.fields
                    .into_iter()
                    .map(|(identifier, expression)| {
                        (identifier, expression.expect("Missing field expression"))
                    })
                    .collect::<Vec<(Identifier, Expression)>>(),
            )
    }
}
