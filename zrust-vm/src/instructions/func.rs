extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::{Call, Return};

impl<E, O> VMInstruction<E, O> for Call
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.call(self.address, self.inputs_count)
    }
}

impl<E, O> VMInstruction<E, O> for Return
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.ret(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zrust_bytecode::*;
    use crate::instructions::testing_utils::{VMTestRunner, TestingError};

    #[test]
    fn test_func() -> Result<(), TestingError> {
        env_logger::builder().is_test(true).try_init();

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
