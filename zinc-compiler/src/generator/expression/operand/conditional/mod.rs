//!
//! The generator expression conditional operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::bytecode::Bytecode;
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

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        self.condition.write_all_to_bytecode(bytecode.clone());
        bytecode.borrow_mut().push_instruction(
            Instruction::If(zinc_bytecode::If),
            crate::lexical::Location::default(),
        );
        self.main_block.write_all_to_bytecode(bytecode.clone());
        if let Some(else_block) = self.else_block {
            bytecode.borrow_mut().push_instruction(
                Instruction::Else(zinc_bytecode::Else),
                crate::lexical::Location::default(),
            );
            else_block.write_all_to_bytecode(bytecode.clone());
        }
        bytecode.borrow_mut().push_instruction(
            Instruction::EndIf(zinc_bytecode::EndIf),
            crate::lexical::Location::default(),
        );
    }
}
