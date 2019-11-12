//use crate::{RuntimeError, Stack};
//use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
//use bellman::pairing::Engine;
//use ff::Field;
//use crate::stack::Primitive;
//use zrust_bytecode::instructions::Lt;
//use crate::vm_instruction::VMInstruction;
//use crate::instructions::utils;
//
//impl<E, CS> VMInstruction<E, CS> for Lt where E: Engine, CS: ConstraintSystem<E> {
//    fn execute(
//        &self,
//        cs: &mut CS,
//        stack: &mut Stack<E>)
//        -> Result<(), RuntimeError>
//    {
//        let left = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
//        let right = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
//
//        let lt_value = match (left.value, right.value) {
//            (Some(a), Some(b)) => {
//                if utils::fr_to_bigint(&a) < utils::fr_to_bigint(&b) {
//                    Some(E::Fr::one())
//                } else {
//                    Some(E::Fr::zero())
//                }
//            }
//            _ => None
//        };
//
//        let and_var = cs.alloc(
//            || "and",
//            || and_value.ok_or(SynthesisError::AssignmentMissing)
//        ).map_err(|_| RuntimeError::SynthesisError)?;
//
//        cs.enforce(
//            || "equality",
//            |lc| lc + left.variable,
//            |lc| lc + right.variable,
//            |lc| lc + and_var
//        );
//
//        stack.push(Primitive { value: and_value, variable: and_var });
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
//    fn test_lt() -> Result<(), RuntimeError> {
//        let mut bytecode = testing_utils::create_instructions_vec();
//        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
//        bytecode.push(Box::new(And));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
//        bytecode.push(Box::new(And));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x00) }));
//        bytecode.push(Box::new(And));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
//        bytecode.push(Box::new(Push { value: BigInt::from(0x01) }));
//        bytecode.push(Box::new(And));
//
//        let stack = testing_utils::execute(bytecode.as_slice())?;
//
//        assert_eq!(stack.len(), 4);
//        testing_utils::assert_stack_value(&stack, 0, "0x01");
//        testing_utils::assert_stack_value(&stack, 1, "0x00");
//        testing_utils::assert_stack_value(&stack, 2, "0x00");
//        testing_utils::assert_stack_value(&stack, 3, "0x00");
//
//        Ok(())
//    }
//}
