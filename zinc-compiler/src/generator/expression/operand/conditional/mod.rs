//!
//! The generator expression conditional operand.
//!

pub mod builder;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;

#[derive(Debug, Clone)]
pub struct Expression {
    condition: GeneratorExpression,
    main_block: BlockExpression,
    else_block: Option<BlockExpression>,
}

impl Expression {
    pub fn new(
        condition: GeneratorExpression,
        main_block: BlockExpression,
        else_block: Option<BlockExpression>,
    ) -> Self {
        Self {
            condition,
            main_block,
            else_block,
        }
    }

    fn _temp() {
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::If(zinc_bytecode::If), location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::Else(zinc_bytecode::Else), else_block.location);
        // self.bytecode
        //     .borrow_mut()
        //     .push_instruction(Instruction::EndIf(zinc_bytecode::EndIf), location);
    }
}
