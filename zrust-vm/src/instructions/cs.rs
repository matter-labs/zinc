extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::ConditionalSelect;

impl<E, O> VMInstruction<E, O> for ConditionalSelect
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let condition = vm.stack_pop()?;
        let if_true = vm.stack_pop()?;
        let if_false = vm.stack_pop()?;

        let selected = vm.get_operator().conditional_select(condition, if_true, if_false)?;

        vm.stack_push(selected)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_cs() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(1337) }));
        bytecode.push(Box::new(Push { value: BigInt::from(42) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0) }));
        bytecode.push(Box::new(ConditionalSelect));
        bytecode.push(Box::new(Push { value: BigInt::from(420) }));
        bytecode.push(Box::new(Push { value: BigInt::from(69) }));
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(ConditionalSelect));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[69, 1337]);

        Ok(())
    }
}
