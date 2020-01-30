use crate::gadgets::utils::bigint_to_fr;
use crate::gadgets::{Gadget, Primitive, ScalarType};
use crate::RuntimeError;
use crate::ZincEngine;
use bellman::{ConstraintSystem, SynthesisError};
use ff::{Field, PrimeField};
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::num::AllocatedNum;
use num_bigint::BigInt;

pub struct SignedFromBits;

impl<E: ZincEngine> Gadget<E> for SignedFromBits {
    type Input = Vec<Primitive<E>>;
    type Output = Primitive<E>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let (data_type, length) = if input.len() == (E::Fr::NUM_BITS as usize) {
            (None, E::Fr::NUM_BITS as usize)
        } else {
            assert_eq!(
                input.len() % 8,
                0,
                "Scalar bit length should be multiple of 8"
            );
            let data_type = ScalarType {
                signed: false,
                length: input.len(),
            };
            (Some(data_type), input.len())
        };

        let mut bits = Vec::with_capacity(length);
        for (i, value) in input.iter().enumerate() {
            let bit = value.value.map(|fr| -> bool { !fr.is_zero() });
            let allocated_bit =
                AllocatedBit::alloc(cs.namespace(|| format!("AllocatedBit {}", i)), bit)?;
            bits.push(allocated_bit.into());
        }

        let adjusted_num =
            AllocatedNum::pack_bits_to_element(cs.namespace(|| "pack_bits_to_element"), &bits)?;

        let adjustment_bigint = BigInt::from(1) << (input.len() - 1);
        let adjustment_fr: E::Fr = bigint_to_fr::<E>(&adjustment_bigint).expect("too much bits");
        let value = match adjusted_num.get_value() {
            None => None,
            Some(mut fr) => {
                fr.sub_assign(&adjustment_fr);
                Some(fr)
            }
        };

        let variable = cs.alloc(
            || "variable",
            || value.ok_or(SynthesisError::AssignmentMissing),
        )?;

        cs.enforce(
            || "hello",
            |zero| zero + variable + (adjustment_fr, CS::one()),
            |zero| zero + CS::one(),
            |zero| zero + adjusted_num.get_variable(),
        );

        Ok(Primitive {
            value,
            variable,
            data_type,
        })
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        vec![output]
    }
}
