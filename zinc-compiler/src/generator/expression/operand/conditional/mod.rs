//!
//! The generator expression conditional operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::Instruction;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::lexical::token::location::Location;

///
/// The conditional expression which is translated to a Zinc VM conditional.
///
#[derive(Debug, Clone)]
pub struct Expression {
    /// The conditional expression location.
    location: Location,
    /// The condition expression.
    condition: GeneratorExpression,
    /// The main block expression.
    main_block: BlockExpression,
    /// The `else`-block expression.
    else_block: Option<BlockExpression>,
}

impl Expression {
    ///
    /// A shortcut constructor.
    ///
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
}

impl IBytecodeWritable for Expression {
    fn write_all(self, bytecode: Rc<RefCell<State>>) {
        self.condition.write_all(bytecode.clone());
        bytecode
            .borrow_mut()
            .push_instruction(Instruction::If(zinc_build::If), Some(self.location));
        self.main_block.write_all(bytecode.clone());

        if let Some(else_block) = self.else_block {
            bytecode
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_build::Else), Some(self.location));
            else_block.write_all(bytecode.clone());
        }

        bytecode
            .borrow_mut()
            .push_instruction(Instruction::EndIf(zinc_build::EndIf), Some(self.location));
    }
}
