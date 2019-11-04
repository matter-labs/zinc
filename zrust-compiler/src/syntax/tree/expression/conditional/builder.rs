//!
//! The conditional expression builder.
//!

use crate::lexical::Location;
use crate::syntax::BlockExpression;
use crate::syntax::ConditionalExpression;
use crate::syntax::Expression;

#[derive(Default)]
pub struct Builder {
    location: Option<Location>,
    condition: Option<Expression>,
    main_block: Option<BlockExpression>,
    else_if: Option<ConditionalExpression>,
    else_block: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, value: Location) {
        self.location = Some(value);
    }

    pub fn set_condition(&mut self, value: Expression) {
        self.condition = Some(value);
    }

    pub fn set_main_block(&mut self, value: BlockExpression) {
        self.main_block = Some(value);
    }

    pub fn set_else_if(&mut self, value: ConditionalExpression) {
        self.else_if = Some(value);
    }

    pub fn set_else_block(&mut self, value: BlockExpression) {
        self.else_block = Some(value);
    }

    pub fn finish(mut self) -> ConditionalExpression {
        ConditionalExpression::new(
            self.location.take().expect("Missing location"),
            self.condition.take().expect("Missing condition"),
            self.main_block.take().expect("Missing main block"),
            self.else_if.take(),
            self.else_block.take(),
        )
    }
}
