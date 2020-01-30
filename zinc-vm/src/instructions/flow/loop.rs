extern crate franklin_crypto;

use crate::gadgets::Gadgets;
use crate::core::{InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use crate::Engine;
use zinc_bytecode::{LoopBegin, LoopEnd};
use self::franklin_crypto::bellman::ConstraintSystem;

impl<E, CS> VMInstruction<E, CS> for LoopBegin
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations)
    }
}

impl<E, CS> VMInstruction<E, CS> for LoopEnd
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::{Add, Load, PushConst, Store};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst::new_untyped(0.into()))
            .add(Store::new(0))
            .add(PushConst::new_untyped(0.into()))
            .add(Store::new(1))
            .add(LoopBegin::new(10))
            .add(Load::new(0))
            .add(PushConst::new_untyped(1.into()))
            .add(Add)
            .add(Store::new(0))
            .add(Load::new(0))
            .add(Load::new(1))
            .add(Add)
            .add(Store::new(1))
            .add(LoopEnd)
            .add(Load::new(0))
            .add(Load::new(1))
            .test(&[55, 10])
    }
}
