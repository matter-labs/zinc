extern crate franklin_crypto;


use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Eq;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Eq
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let eq = vm.operations().eq(left, right)?;

        vm.push(Cell::Value(eq))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_eq() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Eq)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Eq)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Eq)
            .test(&[0, 1, 0])
    }
}
