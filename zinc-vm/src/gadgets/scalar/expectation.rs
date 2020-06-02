use ff::PrimeField;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::IEngine;

pub trait ITypeExpectation: Sized {
    fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError>;
    fn assert_type(&self, expected: Self) -> Result<(), RuntimeError>;
    fn assert_signed(&self, is_signed: bool) -> Result<(), RuntimeError>;
    fn bit_length<E: IEngine>(&self) -> usize;
}

impl ITypeExpectation for ScalarType {
    fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError> {
        if left == right {
            Ok(left)
        } else {
            Err(RuntimeError::TypeError {
                expected: left.to_string(),
                actual: right.to_string(),
            })
        }
    }

    fn assert_type(&self, expected: Self) -> Result<(), RuntimeError> {
        if self == &expected {
            Ok(())
        } else {
            Err(RuntimeError::TypeError {
                expected: expected.to_string(),
                actual: self.to_string(),
            })
        }
    }

    fn assert_signed(&self, is_signed: bool) -> Result<(), RuntimeError> {
        let ok = match self {
            ScalarType::Field | ScalarType::Boolean => false,
            ScalarType::Integer(int_type) => int_type.is_signed == is_signed,
        };

        if ok {
            Ok(())
        } else {
            Err(RuntimeError::TypeError {
                expected: if is_signed {
                    "signed integer".to_string()
                } else {
                    "unsigned integer".to_string()
                },
                actual: self.to_string(),
            })
        }
    }

    fn bit_length<E: IEngine>(&self) -> usize {
        match self {
            ScalarType::Field => E::Fr::NUM_BITS as usize,
            ScalarType::Boolean => 1,
            ScalarType::Integer(inner) => inner.bitlength,
        }
    }
}
