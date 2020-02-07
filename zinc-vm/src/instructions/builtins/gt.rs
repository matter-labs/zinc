extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Gt;

impl<E, CS> VMInstruction<E, CS> for Gt
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let gt = vm.operations().gt(left, right)?;

        vm.push(Cell::Value(gt))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_gt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new(2.into(), true, 8))
            .add(PushConst::new(1.into(), true, 8))
            .add(Gt)
            .add(PushConst::new(2.into(), true, 8))
            .add(PushConst::new(2.into(), true, 8))
            .add(Gt)
            .add(PushConst::new(1.into(), true, 8))
            .add(PushConst::new(2.into(), true, 8))
            .add(Gt)
            .test(&[0, 0, 1])
    }
}
