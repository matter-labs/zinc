extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM, RuntimeError, VirtualMachine};
use zrust_bytecode::{LoopBegin, LoopEnd};

impl<E, O> VMInstruction<E, O> for LoopBegin
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations)
    }
}

impl<E, O> VMInstruction<E, O> for LoopEnd
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::{Add, PushConst, LoadPush, PopStore};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(PushConst { value: 0.into() })
            .add(PopStore::new(0))
            .add(PushConst { value: 0.into() })
            .add(PopStore::new(1))

            .add(LoopBegin::new(10))
                .add(LoadPush::new(0))
                .add(PushConst { value: 1.into() })
                .add(Add)
                .add(PopStore::new(0))
                .add(LoadPush::new(0))
                .add(LoadPush::new(1))
                .add(Add)
                .add(PopStore::new(1))
            .add(LoopEnd)

            .add(LoadPush::new(0))
            .add(LoadPush::new(1))
            .test(&[55, 10])
    }
}
