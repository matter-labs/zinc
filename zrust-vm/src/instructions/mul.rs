extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Mul;

impl<E, O> VMInstruction<E, O> for Mul
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
        let prod = vm.get_operator().mul(left, right)?;

        vm.stack_push(prod)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_mul() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x03) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Mul));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[0x0C]);

        Ok(())
    }
}
