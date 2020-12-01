//!
//! The `Add` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Add;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Add {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let sum_type = zinc_types::ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_sum = gadgets::arithmetic::add::add(cs.namespace(|| "sum"), &left, &right)?;

        let sum = Scalar::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_sum,
            sum_type,
        )?;

        vm.push(Cell::Value(sum))
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;
    use num::One;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_add() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new_field(BigInt::one()))
            .push(zinc_types::Push::new_field(BigInt::from(2)))
            .push(zinc_types::Add)
            .test(&[3])
    }
}
