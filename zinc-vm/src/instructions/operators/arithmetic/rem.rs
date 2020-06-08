use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Rem;
use zinc_bytecode::ScalarType;

use crate::core::execution_state::cell::Cell;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::expectation::ITypeExpectation;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Rem {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
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

        let rem = gadgets::types::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_rem,
            ScalarType::expect_same(left.get_type(), right.get_type())?,
        )?;

        vm.push(Cell::Value(rem))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        TestRunner::new()
            .add(zinc_bytecode::Push::new(9.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(4.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Rem)
            .add(zinc_bytecode::Push::new(9.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Push::new(
                (-4).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Rem)
            .add(zinc_bytecode::Push::new(
                (-9).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new(4.into(), IntegerType::I8.into()))
            .add(zinc_bytecode::Rem)
            .add(zinc_bytecode::Push::new(
                (-9).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Push::new(
                (-4).into(),
                IntegerType::I8.into(),
            ))
            .add(zinc_bytecode::Rem)
            .test(&[3, 3, 1, 1])
    }
}
