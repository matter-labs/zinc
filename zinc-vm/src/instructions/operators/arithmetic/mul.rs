//!
//! The `Mul` instruction.
//!

use franklin_crypto::bellman::ConstraintSystem;

use zinc_types::Mul;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Mul {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let mul_type = zinc_types::ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_mul = gadgets::arithmetic::mul::mul(cs.namespace(|| "mul"), &left, &right)?;

        let mul = Scalar::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_mul,
            mul_type,
        )?;

        vm.push(Cell::Value(mul))
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_mul() -> Result<(), TestingError> {
        TestRunner::new()
            .push(zinc_types::Push::new_field(BigInt::from(3)))
            .push(zinc_types::Push::new_field(BigInt::from(4)))
            .push(zinc_types::Mul)
            .test(&[12])
    }
}
