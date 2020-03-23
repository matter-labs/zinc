//!
//! The generator expression constant operand.
//!

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::scalar::ScalarType;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;

use crate::semantic::Constant as SemanticConstant;

#[derive(Debug, Clone)]
pub struct Constant {
    pub value: BigInt,
    pub is_signed: bool,
    pub bitlength: usize,
}

impl Constant {
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    pub fn try_from_semantic(constant: &SemanticConstant) -> Option<Self> {
        match constant {
            SemanticConstant::Boolean(boolean) => Some(Self::new(
                if *boolean {
                    BigInt::one()
                } else {
                    BigInt::zero()
                },
                false,
                crate::BITLENGTH_BOOLEAN,
            )),
            SemanticConstant::Integer(integer) => Some(Self::new(
                integer.value.to_owned(),
                integer.is_signed,
                integer.bitlength,
            )),
            _ => None,
        }
    }

    pub fn into_instruction(self) -> Instruction {
        let scalar_type = match (self.is_signed, self.bitlength) {
            (false, crate::BITLENGTH_BOOLEAN) => ScalarType::Boolean,
            (false, crate::BITLENGTH_FIELD) => ScalarType::Field,
            (is_signed, bitlength) => ScalarType::Integer(IntegerType {
                is_signed,
                bitlength,
            }),
        };
        Instruction::PushConst(PushConst::new(self.value, scalar_type))
    }
}
