extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::{LoopBegin, LoopEnd};

impl<E, O> VMInstruction<E, O> for LoopBegin
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_begin(self.iterations, self.io_size)
    }
}

impl<E, O> VMInstruction<E, O> for LoopEnd
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.loop_end()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use zrust_bytecode::{Push, Add};
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_loop() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            .add(Push { value: 42.into() })
            .add(Push { value: 0.into() })
            .add(LoopBegin::new(10, 1))
            .add(Push { value: 1.into() })
            .add(Add)
            .add(LoopEnd)

            .test(&[10, 42])
    }
}
