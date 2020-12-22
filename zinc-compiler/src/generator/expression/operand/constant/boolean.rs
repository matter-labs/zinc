//!
//! The generator expression boolean constant operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num::BigInt;
use num::One;
use num::Zero;

use zinc_types::Instruction;
use zinc_types::Push;

use crate::generator::zinc_vm::State as ZincVMState;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::constant::boolean::Boolean as SemanticBooleanConstant;

///
/// The generator expression boolean constant operand.
///
#[derive(Debug, Clone)]
pub struct Boolean {
    /// The inner value.
    pub inner: bool,
}

impl Boolean {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(inner: bool) -> Self {
        Self { inner }
    }

    ///
    /// Converts from the semantic boolean constant.
    ///
    pub fn from_semantic(boolean: &SemanticBooleanConstant) -> Self {
        Self::new(boolean.inner)
    }
}

impl IBytecodeWritable for Boolean {
    fn write_to_zinc_vm(self, state: Rc<RefCell<ZincVMState>>) {
        let value = if self.inner {
            BigInt::one()
        } else {
            BigInt::zero()
        };

        state.borrow_mut().push_instruction(
            Instruction::Push(Push::new(value, zinc_types::ScalarType::Boolean)),
            None,
        );
    }
}
