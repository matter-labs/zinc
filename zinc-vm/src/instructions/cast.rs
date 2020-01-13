extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Cast;

impl<E, O> VMInstruction<E, O> for Cast
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, _vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        Ok(())
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
