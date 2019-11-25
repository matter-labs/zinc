extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::VMInstruction;
use crate::vm::{RuntimeError, VirtualMachine};
use zrust_bytecode::instructions::Rem;

impl<E, O> VMInstruction<E, O> for Rem
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
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
    use num_bigint::BigInt;
    use zrust_bytecode::*;

    #[test]
    fn test_rem() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push {
            value: BigInt::from(4),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(9),
        }));
        bytecode.push(Box::new(Rem));
        bytecode.push(Box::new(Push {
            value: BigInt::from(-4),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(9),
        }));
        bytecode.push(Box::new(Rem));
        bytecode.push(Box::new(Push {
            value: BigInt::from(4),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(-9),
        }));
        bytecode.push(Box::new(Rem));
        bytecode.push(Box::new(Push {
            value: BigInt::from(-4),
        }));
        bytecode.push(Box::new(Push {
            value: BigInt::from(-9),
        }));
        bytecode.push(Box::new(Rem));

        let mut vm = testing_utils::new_test_constrained_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[3, 3, 1, 1]);

        let cs = vm.get_operator().constraint_system();
        assert_eq!(cs.find_unconstrained(), "", "unconstrained variables");
        assert!(cs.is_satisfied(), "satisfied");

        Ok(())
    }
}
