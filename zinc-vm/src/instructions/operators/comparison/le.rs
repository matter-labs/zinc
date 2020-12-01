//!
//! The `LesserOrEquals` instruction.
//!

use num::bigint::ToBigInt;

use zinc_types::Le;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Le {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();
        let left = vm.pop()?.try_into_value()?.to_bigint().unwrap_or_default();

        vm.push(Cell::Value(Scalar::new_constant_bool(left <= right)))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Le)
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Le)
            .push(zinc_types::Push::new(
                BigInt::one(),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Le)
            .push(zinc_types::Push::new(
                BigInt::from(-2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Le)
            .push(zinc_types::Push::new(
                BigInt::from(2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(-2),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
