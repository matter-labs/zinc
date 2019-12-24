extern crate franklin_crypto;

use crate::primitive::{Primitive, PrimitiveOperations};
use crate::vm::{VMInstruction, InternalVM, RuntimeError, VirtualMachine};
use zinc_bytecode::{LoopBegin, LoopEnd};

impl<E, O> VMInstruction<E, O> for LoopBegin
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations)
    }
}

impl<E, O> VMInstruction<E, O> for LoopEnd
where
    E: Primitive,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::{Add, PushConst, Load, Store};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(Store::new(0))
            .add(PushConst { value: 0.into() })
            .add(Store::new(1))

            .add(LoopBegin::new(10))
                .add(Load::new(0))
                .add(PushConst { value: 1.into() })
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
