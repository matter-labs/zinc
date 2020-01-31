extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Sub;

impl<E, CS> VMInstruction<E, CS> for Sub
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let diff = vm.operations().sub(left, right)?;

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
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Sub)
            .test(&[1])
    }
}
