//!
//! The generator expression constant operand.
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
use crate::generator::r#type::Type;
use crate::semantic::element::constant::Constant as SemanticConstant;

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Constant {
    pub fn new_boolean(value: bool) -> Self {
        Self {
            value: if value { BigInt::one() } else { BigInt::zero() },
            is_signed: false,
            bitlength: crate::BITLENGTH_BOOLEAN,
        }
    }

    pub fn new_integer(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
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

    pub fn try_from_semantic(constant: &SemanticConstant) -> Option<Self> {
        match constant {
            SemanticConstant::Boolean(boolean) => Some(Self::new_boolean(boolean.inner)),
            SemanticConstant::Integer(integer) => Some(Self::new_integer(
                integer.value.to_owned(),
                integer.is_signed,
                integer.bitlength,
            )),
            _ => None,
        }
    }

    pub fn r#type(&self) -> Type {
        match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_BOOLEAN) => Type::boolean(),
            (false, crate::BITLENGTH_FIELD) => Type::field(),
            (is_signed, bitlength) => Type::integer(is_signed, bitlength),
        }
    }

    pub fn write_all_to_bytecode(self, bytecode: Rc<RefCell<Bytecode>>) {
        let scalar_type = match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_BOOLEAN) => ScalarType::Boolean,
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
