//!
//! The generator expression conditional operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_bytecode::Instruction;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;
use crate::lexical::token::location::Location;

///
/// The conditional expression which is translated to a Zinc VM conditional.
///
#[derive(Debug, Clone)]
pub struct Expression {
    location: Location,
    condition: GeneratorExpression,
    main_block: BlockExpression,
    else_block: Option<BlockExpression>,
}

impl Expression {
    pub fn new(
        location: Location,
        condition: GeneratorExpression,
        main_block: BlockExpression,
        else_block: Option<BlockExpression>,
    ) -> Self {
        Self {
            location,
            condition,
            main_block,
            else_block,
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<State>>) {
        self.condition.write_all_to_bytecode(bytecode.clone());
        bytecode
            .borrow_mut()
            .push_instruction(Instruction::If(zinc_bytecode::If), Some(self.location));
        self.main_block.write_all_to_bytecode(bytecode.clone());

        if let Some(else_block) = self.else_block {
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_bytecode::Else), Some(self.location));
            else_block.write_all_to_bytecode(bytecode.clone());
        }

        bytecode.borrow_mut().push_instruction(
            Instruction::EndIf(zinc_bytecode::EndIf),
            Some(self.location),
        );
    }
}
