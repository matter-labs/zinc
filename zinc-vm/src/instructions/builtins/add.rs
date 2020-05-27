extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::RuntimeError;
use crate::core::{Cell, VMInstruction, VirtualMachine};
use crate::gadgets;
use crate::gadgets::{ScalarType, ScalarTypeExpectation};

use zinc_bytecode::instructions::Add;

impl<VM: VirtualMachine> VMInstruction<VM> for Add {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let sum_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_sum = gadgets::add(cs.namespace(|| "sum"), &left, &right)?;

        let sum = gadgets::types::conditional_type_check(
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
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_add() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(1.into()))
            .add(PushConst::new_field(2.into()))
            .add(Add)
            .test(&[3])
    }
}
