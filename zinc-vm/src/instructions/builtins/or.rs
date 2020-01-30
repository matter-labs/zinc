extern crate franklin_crypto;

use crate::gadgets::Gadgets;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Or;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Or
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let or = vm.operations().or(left, right)?;

        vm.push(Cell::Value(or))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_or() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(Or)
            .add(PushConst::new_untyped(0.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Or)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(0.into()))
            .add(Or)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Or)
            .test(&[1, 1, 1, 0])
    }
}
