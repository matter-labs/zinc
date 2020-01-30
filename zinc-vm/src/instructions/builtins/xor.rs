extern crate franklin_crypto;


use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Xor;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Xor
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let xor = vm.operations().xor(left, right)?;

        vm.push(Cell::Value(xor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_xor() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(Xor)
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Xor)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(Xor)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Xor)
            .test(&[0, 1, 1, 0])
    }
}
