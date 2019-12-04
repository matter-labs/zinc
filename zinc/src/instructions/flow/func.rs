extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::{Call, Return};

impl<E, O> VMInstruction<E, O> for Call
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.call(self.address, self.inputs_count)
    }
}

impl<E, O> VMInstruction<E, O> for Return
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.ret(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zinc_bytecode::*;

    #[test]
    fn test_func() -> Result<(), TestingError> {
        let _ = env_logger::builder().is_test(true).try_init();

        VMTestRunner::new()
            // call main
            .add(Call::new(9, 0))
            // func min(field, field) -> field
            .add(LoadPush::new(1))
            .add(LoadPush::new(0))
            .add(LoadPush::new(1))
            .add(LoadPush::new(0))
            .add(Lt)
            .add(ConditionalSelect)
            .add(Return::new(1))
            // func main
            .add(PushConst { value: 42.into() })
            .add(PushConst { value: 5.into() })
            .add(PushConst { value: 3.into() })
            .add(Call::new(2, 2))
            .test(&[3, 42])
    }
}
