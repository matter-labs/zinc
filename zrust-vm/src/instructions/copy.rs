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
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(3) }));
        bytecode.push(Box::new(Copy::new(0)));
        bytecode.push(Box::new(Copy::new(2)));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[3, 1, 3, 2, 1]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
