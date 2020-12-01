//!
//! The `Rem` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Rem;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Rem {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let (_div, unchecked_rem) = gadgets::arithmetic::div_rem::div_rem_conditional(
            cs.namespace(|| "div_rem"),
            &condition,
            &left,
            &right,
        )?;

        let rem = Scalar::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_rem,
            zinc_types::ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new(
                BigInt::from(9),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(4),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Rem)
            .push(zinc_types::Push::new(
                BigInt::from(9),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(-4),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Rem)
            .push(zinc_types::Push::new(
                BigInt::from(-9),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(4),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Rem)
            .push(zinc_types::Push::new(
                BigInt::from(-9),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Push::new(
                BigInt::from(-4),
                zinc_types::IntegerType::I8.into(),
            ))
            .push(zinc_types::Rem)
            .test(&[3, 3, 1, 1])
    }
}
