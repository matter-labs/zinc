extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::{gadgets};
use zinc_bytecode::instructions::And;

impl<VM: VirtualMachine> VMInstruction<VM> for And
{
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let cs = vm.constraint_system();
        let and = gadgets::boolean::and(cs.namespace(|| "and"), &left, &right)?;

        vm.push(Cell::Value(and))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gadgets::ScalarType;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_and() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(And)
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(And)
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(PushConst::new(0.into(), ScalarType::Boolean))
            .add(And)
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(PushConst::new(1.into(), ScalarType::Boolean))
            .add(And)
            .test(&[1, 0, 0, 0])
    }
}
