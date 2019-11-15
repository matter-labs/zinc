extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::{Call, Return};

impl<E, O> VMInstruction<E, O> for Call
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.function_call(self.address)
    }
}

impl<E, O> VMInstruction<E, O> for Return
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        vm.function_return()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use num_bigint::BigInt;
    use zrust_bytecode::*;

    #[test]
    fn test_func() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        /* 00 */ bytecode.push(Box::new(Call::new(4)));

        // func double(field) -> field
        /* 01 */ bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        /* 02 */ bytecode.push(Box::new(Mul));
        /* 03 */ bytecode.push(Box::new(Return));

        // func main
        /* 04 */ bytecode.push(Box::new(Push { value: BigInt::from(10) }));
        /* 05 */ bytecode.push(Box::new(Call::new(1)));
        /* 06 */ bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        /* 07 */ bytecode.push(Box::new(Add));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[22]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
