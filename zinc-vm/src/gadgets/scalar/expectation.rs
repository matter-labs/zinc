use ff::PrimeField;

use zinc_bytecode::ScalarType;

use crate::error::RuntimeError;
use crate::IEngine;

pub trait ITypeExpectation: Sized {
    fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError>;

    fn assert_type(&self, expected: Self) -> Result<(), RuntimeError>;

    fn assert_signed(&self, is_signed: bool) -> Result<(), RuntimeError>;

    fn bitlength<E: IEngine>(&self) -> usize;
}

impl ITypeExpectation for ScalarType {
    fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError> {
        if left != right {
            return Err(RuntimeError::TypeError {
                expected: left.to_string(),
                actual: right.to_string(),
            });
        }

        Ok(left)
    }

    fn assert_type(&self, expected: Self) -> Result<(), RuntimeError> {
        if self != &expected {
            return Err(RuntimeError::TypeError {
                expected: expected.to_string(),
                actual: self.to_string(),
            });
        }

        Ok(())
    }

    fn assert_signed(&self, is_signed: bool) -> Result<(), RuntimeError> {
        let is_signed = match self {
            ScalarType::Field | ScalarType::Boolean => false,
            ScalarType::Integer(int_type) => int_type.is_signed == is_signed,
        };

        if !is_signed {
            return Err(RuntimeError::TypeError {
                expected: if is_signed {
                    "signed integer".to_string()
                } else {
                    "unsigned integer".to_string()
                },
                actual: self.to_string(),
            });
        }

        Ok(())
    }

    fn bitlength<E: IEngine>(&self) -> usize {
        match self {
            ScalarType::Boolean => 1,
            ScalarType::Integer(inner) => inner.bitlength,
            ScalarType::Field => E::Fr::NUM_BITS as usize,
        }
    }
}
