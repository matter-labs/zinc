use crate::core::EvaluationStack;
use crate::gadgets::{utils, Scalar};
use crate::stdlib::NativeFunction;
use crate::{Engine, MalformedBytecode, Result};

use bellman::ConstraintSystem;
use ff::PrimeField;
use num_bigint::BigInt;

use franklin_crypto::circuit::expression::Expression;
use franklin_crypto::circuit::num::AllocatedNum;
use zinc_bytecode::scalar::IntegerType;

pub struct SignedFromBits {
    bit_length: usize,
}

impl SignedFromBits {
    pub fn new(inputs_count: usize) -> Self {
        Self {
            bit_length: inputs_count,
        }
    }
}

impl<E: Engine> NativeFunction<E> for SignedFromBits {
    fn execute<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result {
        if self.bit_length >= E::Fr::CAPACITY as usize {
            return Err(MalformedBytecode::InvalidArguments(format!(
                "signed_from_bits: integer type with length {} is not supported",
                self.bit_length
            ))
            .into());
        }

        let mut bits = Vec::with_capacity(self.bit_length);
        for i in 0..self.bit_length {
            let bit = stack.pop()?.value()?;
            let boolean = bit.to_boolean(cs.namespace(|| format!("to_boolean {}", i)))?;
            bits.push(boolean);
        }

        let sign_bit = bits[self.bit_length - 1].clone();
        bits.push(sign_bit.not());

        let num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        let num_expr = Expression::from(&num);
        let base_value = BigInt::from(1) << self.bit_length;
        let base_expr = Expression::<E>::constant::<CS>(
            utils::bigint_to_fr::<E>(&base_value).expect("length is too big"),
        );

        let num = (num_expr - base_expr).into_number(cs.namespace(|| "result"))?;

        let int_type = IntegerType {
            signed: true,
            length: self.bit_length,
        };

        let scalar =
            Scalar::new_unchecked_variable(num.get_value(), num.get_variable(), int_type.into());

        stack.push(scalar.into())?;

        Ok(())
    }
}
