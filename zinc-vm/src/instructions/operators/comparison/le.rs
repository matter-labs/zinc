//!
//! The `LesserOrEquals` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::Le;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Le {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let cs = vm.constraint_system();
        let le = gadgets::comparison::lesser_or_equals(cs.namespace(|| "le"), &left, &right)?;

        vm.push(Cell::Value(le))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;
    use num::One;

    use zinc_build::IntegerType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Le)
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Le)
            .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Le)
            .push(zinc_build::Push::new(
                BigInt::from(-2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Le)
            .push(zinc_build::Push::new(
                BigInt::from(2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Push::new(
                BigInt::from(-2),
                IntegerType::I8.into(),
            ))
            .push(zinc_build::Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
