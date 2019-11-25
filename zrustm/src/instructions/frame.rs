extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::{FrameBegin, FrameEnd};

impl<E, O> VMInstruction<E, O> for FrameBegin
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.frame_push(None)
    }
}

impl<E, O> VMInstruction<E, O> for FrameEnd
    where
        E: Element,
        O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.frame_pop(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_frame() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            // call main
            .add(Call::new(8, 0))
            // func min(field, field) -> field
            .add(Copy::new(1))
            .add(Copy::new(0))
            .add(Copy::new(1))
            .add(Copy::new(0))
            .add(Lt)
            .add(ConditionalSelect)
            .add(Return::new(1))
            // func main
            .add(Push { value: 42.into() })
            .add(Push { value: 5.into() })
            .add(Push { value: 3.into() })
            .add(Call::new(1, 2))
            .test(&[3, 42])
    }
}
