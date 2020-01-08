extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Neg;

impl<E, O> VMInstruction<E, O> for Neg
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let element = vm.pop()?.value()?;
        let neg = vm.get_operator().neg(element)?;

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
