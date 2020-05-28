use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::Rem;
use zinc_bytecode::ScalarType;

use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::scalar::scalar_type::ScalarTypeExpectation;
impl<VM: VirtualMachine> VMInstruction<VM> for Rem {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

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
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    use zinc_bytecode::IntegerType;

    #[test]
    fn test_rem() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
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
