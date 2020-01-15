use crate::gadgets::{Gadget, Primitive, DataType};
use crate::RuntimeError;
use bellman::ConstraintSystem;
use pairing::Engine;
use franklin_crypto::circuit::sha256::sha256;
use franklin_crypto::circuit::num::AllocatedNum;

pub struct Sha256;

impl<E: Engine> Gadget<E> for Sha256 {
    type Input = Vec<Primitive<E>>;
    type Output = Vec<Primitive<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let mut bits = Vec::new();
        for byte in input {
            let byte_num = byte.as_allocated_num(cs.namespace(|| "as_allocated_num"))?;
            let mut byte_bits = byte_num
                .into_bits_le_fixed(cs.namespace(|| "into_bits_le_fixed"), 8)?;
            bits.append(&mut byte_bits)
        }

        let digest = sha256(
            cs.namespace(|| "sha256"),
            bits.as_slice()
        )?;

        assert_eq!(digest.len(), 256);

        let mut digest_bytes = Vec::new();
        for byte_bits in digest.chunks(8) {
            let byte = AllocatedNum::pack_bits_to_element(
                cs.namespace(|| "pack_bits_to_element"),
                byte_bits
            )?;

            digest_bytes.push(Primitive {
                value: byte.get_value(),
                variable: byte.get_variable(),
                data_type: Some(DataType { signed: false, length: 8 })
            });
        }

        Ok(digest_bytes)
    }

    fn input_from_vec(input: &[Primitive<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Primitive<E>> {
        output
    }
}
