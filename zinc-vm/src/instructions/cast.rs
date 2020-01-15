extern crate franklin_crypto;

use crate::gadgets::{PrimitiveOperations, DataType};
use crate::vm::{VMInstruction, InternalVM, Cell};
use crate::vm::{RuntimeError, VirtualMachine};
use pairing::Engine;
use zinc_bytecode::instructions::Cast;

impl<E, O> VMInstruction<E, O> for Cast
where
    E: Engine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;
        let data_type = DataType { signed: self.signed, length: self.length };
        let new_value = vm.operations().set_type(old_value, data_type)?;
        vm.push(Cell::Value(new_value))
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
