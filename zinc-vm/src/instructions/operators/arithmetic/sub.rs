use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::ScalarType;
use zinc_bytecode::Sub;

use crate::auto_const;
use crate::core::state::cell::Cell;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets;
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::scalar::scalar_type::ScalarTypeExpectation;
impl<VM: VirtualMachine> VMInstruction<VM> for Sub {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;

        let diff_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_diff = auto_const!(
            gadgets::arithmetic::sub::sub,
            cs.namespace(|| "diff"),
            &left,
            &right
        )?;

        let diff = gadgets::types::conditional_type_check(
            cs.namespace(|| "type check"),
            &condition,
            &unchecked_diff,
            diff_type,
        )?;

        vm.push(Cell::Value(diff))
    }
}

#[cfg(test)]
mod test {
    use crate::tests::TestingError;
    use crate::tests::VMTestRunner;

    #[test]
    fn test_sub() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(2.into()))
            .add(zinc_bytecode::Push::new_field(1.into()))
            .add(zinc_bytecode::Sub)
            .test(&[1])
    }
}
