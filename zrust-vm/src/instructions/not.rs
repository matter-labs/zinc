//use crate::{RuntimeError, Stack};
//use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
//use bellman::pairing::Engine;
//use ff::Field;
//use crate::stack::Primitive;
//use zrust_bytecode::instructions::Not;
//use crate::vm_instruction::VMInstruction;
//
//impl<E, O> VMInstruction<E, O> for Not
//    where E: Engine, O: ElementOperator<E>
//{
//    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
//        let mut op = vm.get_operator();
//        let value = op.constant_bigint(self.value)?;
//        vm.stack_push(value)?;
//
//        Ok(())
//    }
//}
//
//#[cfg(test)]
//mod test {
//    use super::*;
//    use crate::instructions::testing_utils;
//    use zrust_bytecode::*;
//    use num_bigint::BigInt;
//
//    #[test]
//    fn test_not() -> Result<(), RuntimeError> {
//        let mut bytecode = testing_utils::create_instructions_vec();
//        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
//        bytecode.push(Box::new(Not));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
//        bytecode.push(Box::new(Not));
//
//        let stack = testing_utils::execute(bytecode.as_slice())?;
//
//        assert_eq!(stack.len(), 2);
//        testing_utils::assert_stack_value(&stack, 0, "0x01");
//        testing_utils::assert_stack_value(&stack, 1, "0x00");
//
//        Ok(())
//    }
//}
