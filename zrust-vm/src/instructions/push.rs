extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Push;

impl<E, O> VMInstruction<E, O> for Push
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let op = vm.get_operator();
        let value = op.constant_bigint(&self.value)?;

        vm.stack_push(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;

    #[test]
    fn test_push() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x42) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0xABCD) }));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 3);
        testing_utils::assert_stack_value(&stack, 0, "0xABCD");
        testing_utils::assert_stack_value(&stack, 1, "0x42");
        testing_utils::assert_stack_value(&stack, 2, "0x00");

        Ok(())
    }
}
