extern crate franklin_crypto;

use crate::gadgets::Gadgets;
use crate::core::{InternalVM, VMInstruction};
use crate::core::{RuntimeError, VirtualMachine};
use crate::Engine;
use zinc_bytecode::{Call, Return};
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for Call
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        vm.call(self.address, self.inputs_count)
    }
}

impl<E, CS> VMInstruction<E, CS> for Return
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        vm.ret(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

//    #[test]
//    fn test_func() -> Result<(), TestingError> {
//        let _ = env_logger::builder().is_test(true).try_init();
//
//        VMTestRunner::new()
//            // call main
//            .add(Call::new(9, 0))
//            // func min(field, field) -> field
//            .add(Load::new(0))
//            .add(Load::new(1))
//            .add(Lt)
//            .add(Load::new(0))
//            .add(Load::new(1))
//            .add(ConditionalSelect)
//            .add(Return::new(1))
//            // func main
//            .add(PushConst::new_untyped(42.into()))
//            .add(PushConst::new_untyped(5.into()))
//            .add(PushConst::new_untyped(3.into()))
//            .add(Call::new(2, 2))
//            .test(&[3, 42])
//    }
}
