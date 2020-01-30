extern crate franklin_crypto;


use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Lt;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Lt
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let lt = vm.operations().lt(left, right)?;

        vm.push(Cell::Value(lt))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_lt() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Lt)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Lt)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Lt)
            .test(&[1, 0, 0])
    }
}
