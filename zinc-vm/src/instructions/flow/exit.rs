extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Exit;

impl<E, O> VMInstruction<E, O> for Exit
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.exit(self.outputs_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;
    //    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    //    use zinc_bytecode::*;
    //
    //    #[test]
    //    fn test_exit() -> Result<(), TestingError> {
    //        VMTestRunner::new()
    //            .add(PushConst::new_untyped(1.into()))
    //            .add(Exit::new(0))
    //            .add(PushConst::new_untyped(2.into()))
    //            .test(&[1])
    //    }
}
