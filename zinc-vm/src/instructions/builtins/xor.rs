extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Xor;

impl<E, O> VMInstruction<E, O> for Xor
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
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
