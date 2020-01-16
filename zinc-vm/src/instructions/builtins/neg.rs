extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::Neg;

impl<E, O> VMInstruction<E, O> for Neg
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let element = vm.pop()?.value()?;
        let neg = vm.operations().neg(element)?;

        vm.push(Cell::Value(neg))
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        Ok(())
    }
}
