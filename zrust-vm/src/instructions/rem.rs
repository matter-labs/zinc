extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Rem;

impl<E, O> VMInstruction<E, O> for Rem
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;
        let (_div, rem) = vm.get_operator().div_rem(left, right)?;

        vm.stack_push(rem)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_rem() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x10) }));
        bytecode.push(Box::new(Rem));
        bytecode.push(Box::new(Push { value: BigInt::from(0x4) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x9) }));
        bytecode.push(Box::new(Rem));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[0x01, 0x00]);

        Ok(())
    }
}
