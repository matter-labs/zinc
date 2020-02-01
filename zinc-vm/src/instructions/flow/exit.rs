extern crate franklin_crypto;

use self::franklin_crypto::bellman::ConstraintSystem;
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::instructions::Exit;

impl<E, CS> VMInstruction<E, CS> for Exit
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
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
