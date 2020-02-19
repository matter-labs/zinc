use crate::RuntimeError;
pub use zinc_bytecode::scalar::*;

pub trait ScalarTypeExpectation: Sized {
    fn expect_same(left: Self, right: Self) -> Result<Self, RuntimeError>;
    fn assert_type(&self, expected: Self) -> Result<(), RuntimeError>;
}

impl ScalarTypeExpectation for ScalarType {
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
                expected: self.to_string(),
                actual: expected.to_string()
            })
        }
    }
}
