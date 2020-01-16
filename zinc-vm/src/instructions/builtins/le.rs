extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::Le;

impl<E, O> VMInstruction<E, O> for Le
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.pop()?.value()?;
        let right = vm.pop()?.value()?;

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
            .add(PushConst::new_untyped(1.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped(1.into()))
            .add(Le)
            .add(PushConst::new_untyped(2.into()))
            .add(PushConst::new_untyped((-2).into()))
            .add(Le)
            .add(PushConst::new_untyped((-2).into()))
            .add(PushConst::new_untyped(2.into()))
            .add(Le)
            .test(&[0, 1, 1, 1, 0])
    }
}
