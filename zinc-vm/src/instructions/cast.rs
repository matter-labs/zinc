extern crate franklin_crypto;

use crate::gadgets::{DataType, PrimitiveOperations};
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::instructions::Cast;

impl<E, O> VMInstruction<E, O> for Cast
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let old_value = vm.pop()?.value()?;
        let data_type = DataType {
            signed: self.signed,
            length: self.length,
        };
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
