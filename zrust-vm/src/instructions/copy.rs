extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Copy;

impl<E, O> VMInstruction<E, O> for Copy
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let value = vm.stack_get(self.index)?;
        vm.stack_push(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_copy() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x02) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x03) }));
        bytecode.push(Box::new(Copy { index: 0 }));
        bytecode.push(Box::new(Copy { index: 2 }));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 5);
        testing_utils::assert_stack_value(&stack, 0, "0x02");
        testing_utils::assert_stack_value(&stack, 1, "0x03");
        testing_utils::assert_stack_value(&stack, 2, "0x03");
        testing_utils::assert_stack_value(&stack, 3, "0x02");
        testing_utils::assert_stack_value(&stack, 4, "0x01");

        Ok(())
    }
}
