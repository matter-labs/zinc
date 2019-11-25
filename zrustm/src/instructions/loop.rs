extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::{LoopBegin, LoopEnd};

impl<E, O> VMInstruction<E, O> for LoopBegin
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations, self.io_size)
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
    use zrust_bytecode::{Add, Push, Copy};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(Push { value: 42.into() })
            .add(Push { value: 0.into() })
            .add(LoopBegin::new(10, 1))
            .add(Copy::new(1))
            .add(Push { value: 1.into() })
            .add(Add)
            .add(LoopEnd)
            .test(&[10, 42])
    }
}
