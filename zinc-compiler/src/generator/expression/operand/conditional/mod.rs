//!
//! The generator expression conditional operand.
//!

pub mod builder;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_types::Instruction;

use crate::generator::expression::operand::block::Expression as BlockExpression;
use crate::generator::expression::Expression as GeneratorExpression;
use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use zinc_lexical::Location;

///
/// The conditional expression.
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
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        self.condition.write_to_zinc_vm(state.clone());
        state
            .borrow_mut()
            .push_instruction(Instruction::If(zinc_types::If), Some(self.location));
        self.main_block.write_to_zinc_vm(state.clone());

        if let Some(else_block) = self.else_block {
            state
                .borrow_mut()
                .push_instruction(Instruction::Else(zinc_types::Else), Some(self.location));
            else_block.write_to_zinc_vm(state.clone());
        }

        state
            .borrow_mut()
            .push_instruction(Instruction::EndIf(zinc_types::EndIf), Some(self.location));
    }
}
