extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::auto_const;
use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::gadgets::auto_const::prelude::*;
use crate::gadgets::{ScalarType, ScalarTypeExpectation};
use crate::{gadgets};
use zinc_bytecode::instructions::Mul;

impl<VM: VirtualMachine> VMInstruction<VM> for Mul {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let mul_type = ScalarType::expect_same(left.get_type(), right.get_type())?;

        let condition = vm.condition_top()?;
        let cs = vm.constraint_system();

        let unchecked_mul = auto_const!(
            gadgets::arithmetic::mul,
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
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_mul() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_field(3.into()))
            .add(PushConst::new_field(4.into()))
            .add(Mul)
            .test(&[12])
    }
}
