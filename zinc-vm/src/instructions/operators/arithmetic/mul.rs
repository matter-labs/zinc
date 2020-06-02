use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Mul;
use zinc_bytecode::ScalarType;

use crate::auto_const;
use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Mul {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let mul_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_mul = auto_const!(
            gadgets::arithmetic::mul::mul,
            cs.namespace(|| "mul"),
            &left,
            &right
        )?;

        let mul = gadgets::types::conditional_type_check(
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
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test_mul() -> Result<(), TestingError> {
        TestRunner::new()
            .add(zinc_bytecode::Push::new_field(3.into()))
            .add(zinc_bytecode::Push::new_field(4.into()))
            .add(zinc_bytecode::Mul)
            .test(&[12])
    }
}
