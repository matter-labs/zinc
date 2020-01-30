extern crate franklin_crypto;

use crate::gadgets::Gadgets;
use crate::core::{Cell, InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Le;
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Le
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;

        let le = vm.operations().le(left, right)?;

        vm.push(Cell::Value(le))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_le() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Le)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .add(PushConst::new_untyped((-2).into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped((-2).into()))
            .add(Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
