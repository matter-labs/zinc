use crate::{RuntimeError, Stack};
use franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use bellman::pairing::Engine;
use crate::stack::Primitive;
use crate::vm_instruction::VMInstruction;
use zrust_bytecode::Div;
use crate::instructions::utils;
use num_integer::Integer;

impl<E, CS> VMInstruction<E, CS> for Div where E: Engine, CS: ConstraintSystem<E> {
    fn execute(
        &self,
        cs: &mut CS,
        stack: &mut Stack<E>)
        -> Result<(), RuntimeError>
    {
        let denominator = stack.pop().ok_or(RuntimeError::StackUnderflow)?;
        let nominator = stack.pop().ok_or(RuntimeError::StackUnderflow)?;

        let mut quotient: Option<E::Fr> = None;
        let mut remainder: Option<E::Fr> = None;

        if let (Some(nom), Some(denom)) = (nominator.value, denominator.value) {
            let nom_bi = utils::fr_to_bigint::<E>(&nom);
            let denom_bi = utils::fr_to_bigint::<E>(&denom);

            let (q, r) = nom_bi.div_rem(&denom_bi);

            quotient = utils::bigint_to_fr::<E>(&q);
            remainder = utils::bigint_to_fr::<E>(&r);
        }

        let qutioent_var = cs.alloc(
            || "qutioent",
            || quotient.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|e| RuntimeError::SynthesisError)?;

        let remainder_var = cs.alloc(
            || "remainder",
            || remainder.ok_or(SynthesisError::AssignmentMissing))
            .map_err(|e| RuntimeError::SynthesisError)?;

        cs.enforce(
            || "equality",
            |lc| lc + qutioent_var,
            |lc| lc + denominator.variable,
            |lc| lc + nominator.variable - remainder_var
        );

        stack.push(Primitive { value: quotient, variable: qutioent_var });

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_div() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(0x10) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x04) }));
        bytecode.push(Box::new(Div));
        bytecode.push(Box::new(Push { value: BigInt::from(0x9) }));
        bytecode.push(Box::new(Push { value: BigInt::from(0x4) }));
        bytecode.push(Box::new(Div));

        let stack = testing_utils::execute(bytecode.as_slice())?;

        assert_eq!(stack.len(), 2);
        testing_utils::assert_stack_value(&stack, 0, "0x02");
        testing_utils::assert_stack_value(&stack, 1, "0x04");

        Ok(())
    }
}
