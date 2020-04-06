//!
//! The generator expression conditional operand builder.
//!

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::operand::conditional::Expression as ConditionalExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::lexical::token::location::Location;

#[derive(Debug, Default, Clone)]
pub struct Builder {
    location: Option<Location>,
    condition: Option<GeneratorExpression>,
    main_block: Option<BlockExpression>,
    else_block: Option<BlockExpression>,
}

impl Builder {
    pub fn set_location(&mut self, location: Location) {
        self.location = Some(location);
    }

    pub fn set_condition(&mut self, value: GeneratorExpression) {
        self.condition = Some(value);
    }

    pub fn set_main_block(&mut self, value: BlockExpression) {
        self.main_block = Some(value);
    }

    pub fn set_else_block(&mut self, value: BlockExpression) {
        self.else_block = Some(value);
    }

    pub fn finish(mut self) -> ConditionalExpression {
        let location = self
            .location
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "location"));

        let condition = self
            .condition
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "condition"));

        let main_block = self
            .main_block
            .take()
            .unwrap_or_else(|| panic!("{}{}", crate::PANIC_BUILDER_REQUIRES_VALUE, "main_block"));

        let else_block = self.else_block.take();

        ConditionalExpression::new(location, condition, main_block, else_block)
    }
}
