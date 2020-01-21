extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Not;

impl<E, O> VMInstruction<E, O> for Not
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.pop()?.value()?;

        let not = vm.operations().not(value)?;

        vm.push(Cell::Value(not))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_not() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(Not)
            .add(PushConst::new_untyped(1.into()))
            .add(Not)
            .test(&[0, 1])
    }
}
