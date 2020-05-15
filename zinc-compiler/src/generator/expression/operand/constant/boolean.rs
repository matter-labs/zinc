//!
//! The generator expression boolean constant operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;

use crate::generator::bytecode::Bytecode;
use crate::semantic::element::constant::boolean::Boolean as SemanticBooleanConstant;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub inner: bool,
}

impl Boolean {
    pub fn new(inner: bool) -> Self {
        Self { inner }
    }

    pub fn from_semantic(boolean: &SemanticBooleanConstant) -> Self {
        Self::new(boolean.inner)
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let value = if self.inner {
            BigInt::one()
        } else {
            BigInt::zero()
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::PushConst(PushConst::new(value, ScalarType::Boolean)),
            None,
        );
    }
}
