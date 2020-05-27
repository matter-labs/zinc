extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::auto_const;
use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{ScalarType, ScalarTypeExpectation};
use crate::{gadgets};
use zinc_bytecode::instructions::Sub;

impl<VM: VirtualMachine> VMInstruction<VM> for Sub {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let diff_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_diff = auto_const!(
            gadgets::arithmetic::sub,
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
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_sub() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(2.into()))
            .add(PushConst::new_field(1.into()))
            .add(Sub)
            .test(&[1])
    }
}
