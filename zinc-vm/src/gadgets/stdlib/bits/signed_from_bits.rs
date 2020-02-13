use crate::gadgets::utils::bigint_to_fr;
use crate::gadgets::{Gadget, IntegerType, Primitive, ScalarType};
use crate::Engine;
use crate::RuntimeError;
use bellman::{ConstraintSystem};
use ff::Field;
use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
use franklin_crypto::circuit::num::AllocatedNum;
use num_bigint::BigInt;
use franklin_crypto::circuit::expression::Expression;

pub struct SignedFromBits;

impl<E: Engine> Gadget<E> for SignedFromBits {
    type Input = Vec<Primitive<E>>;
    type Output = Primitive<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        assert_eq!(
            input.len() % 8,
            0,
            "Scalar bit length should be multiple of 8"
        );

        let length = input.len();
        let scalar_type = ScalarType::Integer(IntegerType {
            signed: true,
            length,
        });

        let mut bits: Vec<Boolean> = Vec::with_capacity(length);
        for (i, value) in input.iter().rev().enumerate() {
            let bit = value.value.map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }
        let sign_bit = bits[length - 1].clone();
        bits.push(sign_bit.not());

        let num = AllocatedNum::pack_bits_to_element(
            cs.namespace(|| "pack_bits_to_element"),
            &bits
        )?;

        let num_expr = Expression::from(&num);
        let base_value = BigInt::from(1) << length;
        let base_expr = Expression::<E>::constant::<CS>(
            bigint_to_fr::<E>(&base_value).expect("length is too big")
        );

        let result = (num_expr - base_expr).into_number(
            cs.namespace(|| "result")
        )?;

        Ok(Primitive {
            value: result.get_value(),
            variable: result.get_variable(),
            scalar_type,
        })
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        vec![output]
    }
}
