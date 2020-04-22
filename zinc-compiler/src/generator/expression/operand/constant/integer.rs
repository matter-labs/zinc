//!
//! The generator expression integer constant operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;

use crate::generator::bytecode::Bytecode;
use crate::semantic::element::constant::integer::Integer as SemanticIntegerConstant;

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Integer {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn new_min(is_signed: bool, bitlength: usize) -> Self {
        let value = match (is_signed, bitlength) {
            (false, _bitlength) => BigInt::zero(),
            (true, bitlength) => -(BigInt::one() << (bitlength - 1)),
        };

        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn new_max(is_signed: bool, bitlength: usize) -> Self {
        let value = match (is_signed, bitlength) {
            (false, bitlength) => (BigInt::one() << bitlength) - BigInt::one(),
            (true, bitlength) => (BigInt::one() << (bitlength - 1)) - BigInt::one(),
        };

        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn from_semantic(integer: &SemanticIntegerConstant) -> Self {
        Self::new(
            integer.value.to_owned(),
            integer.is_signed,
            integer.bitlength,
        )
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let scalar_type = match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_FIELD) => ScalarType::Field,
            (is_signed, bitlength) => ScalarType::Integer(IntegerType {
                is_signed,
                bitlength,
            }),
        };

        bytecode.borrow_mut().push_instruction(
            Instruction::PushConst(PushConst::new(self.value, scalar_type)),
            None,
        );
    }
}
