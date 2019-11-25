extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Exit;

impl<E, O> VMInstruction<E, O> for Exit
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.exit(self.outputs_count)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::testing_utils::{TestingError, VMTestRunner};
    use zrust_bytecode::*;

    #[test]
    fn test_exit() -> Result<(), TestingError> {
        VMTestRunner::new()
            .add(Push { value: 1.into() })
            .add(Exit::new(0))
            .add(Push { value: 2.into() })
            .test(&[1])
    }
}
