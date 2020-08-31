//!
//! The generator expression integer constant operand.
//!

use std::cell::RefCell;
use std::rc::Rc;

use num_bigint::BigInt;
use num_traits::One;
use num_traits::Zero;

use zinc_build::Instruction;
use zinc_build::IntegerType;
use zinc_build::Push;
use zinc_build::ScalarType;

use crate::generator::state::State;
use crate::generator::IBytecodeWritable;
use crate::semantic::element::constant::integer::Integer as SemanticIntegerConstant;

///
/// The generator expression integer constant operand.
///
#[derive(Debug, Clone)]
pub struct Integer {
    /// The inner value.
    pub value: BigInt,
    /// Whether the integer type is signed.
    pub is_signed: bool,
    /// The integer type bitlength.
    pub bitlength: usize,
}

impl Integer {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(value: BigInt, is_signed: bool, bitlength: usize) -> Self {
        Self {
            value,
            is_signed,
            bitlength,
        }
    }

    ///
    /// Returns the minimum value for the specified type.
    ///
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

    ///
    /// Returns the maximum value for the specified type.
    ///
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

    ///
    /// Converts from the semantic integer constant.
    ///
    pub fn from_semantic(integer: &SemanticIntegerConstant) -> Self {
        Self::new(
            integer.value.to_owned(),
            integer.is_signed,
            integer.bitlength,
        )
    }
}

impl IBytecodeWritable for Integer {
    fn write_all(self, state: Rc<RefCell<State>>) {
        let scalar_type = match (self.is_signed, self.bitlength) {
            (false, zinc_const::bitlength::FIELD) => ScalarType::Field,
            (is_signed, bitlength) => ScalarType::Integer(IntegerType {
                is_signed,
                bitlength,
            }),
        };

        state
            .borrow_mut()
            .push_instruction(Instruction::Push(Push::new(self.value, scalar_type)), None);
    }
}
