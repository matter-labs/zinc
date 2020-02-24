use crate::gadgets::{Gadget, Scalar};
use crate::Engine;
use crate::RuntimeError;
use bellman::ConstraintSystem;
use ff::Field;
use franklin_crypto::circuit::boolean::AllocatedBit;
use franklin_crypto::circuit::sha256::sha256;
use zinc_bytecode::scalar::ScalarType;

pub struct Sha256;

impl<E: Engine> Gadget<E> for Sha256 {
    type Input = Vec<Scalar<E>>;
    type Output = Vec<Scalar<E>>;

    fn synthesize<CS: ConstraintSystem<E>>(
        &self,
        mut cs: CS,
        input: Self::Input,
    ) -> Result<Self::Output, RuntimeError> {
        let mut bits = Vec::new();
        for (i, bit_scalar) in input.into_iter().enumerate() {
            let allocated_bit = AllocatedBit::alloc(
                cs.namespace(|| format!("AllocatedBit {}", i)),
                bit_scalar.get_value().map(|fr| !fr.is_zero()),
            )?;

            bits.push(allocated_bit.into());
        }

        let digest_bits = sha256(cs.namespace(|| "sha256"), &bits)?;

        assert_eq!(digest_bits.len(), 256);

        let digest = digest_bits
            .into_iter()
            .map(|bit| Scalar::new_unchecked_variable(
                bit.get_value_field::<E>(),
                bit
                    .get_variable()
                    .expect("sha256 must allocate")
                    .get_variable(),
                ScalarType::Boolean,
            ))
            .collect();

        Ok(digest)
    }

    fn input_from_vec(input: &[Scalar<E>]) -> Result<Self::Input, RuntimeError> {
        Ok(Vec::from(input))
    }

    fn output_into_vec(output: Self::Output) -> Vec<Scalar<E>> {
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::constraint_systems::DebugConstraintSystem;
    use franklin_crypto::bellman::ConstraintSystem;
    use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
    use franklin_crypto::circuit::sha256::sha256;
    use pairing::bn256::Bn256;

    #[test]
    fn test_sha256_endiannes() {
        let preimage =
            hex::decode("1d0bf151b6362dea55be4e78b03e01d24b80c52d798b4a7285061e52927b6b").unwrap();
        let expected_digest =
            hex::decode("c905b353e318a4b0e509bb6a0dd2afa55aa292ec0073119f529bdf244edcd17e")
                .unwrap();

        let mut cs = DebugConstraintSystem::<Bn256>::default();

        const IS_PREIMAGE_BE: bool = true;
        const IS_DIGEST_BE: bool = true;

        let preimage_bits: Vec<Boolean> = preimage
            .iter()
            .map(|byte| {
                let mut bits = Vec::with_capacity(8);
                for i in 0..8 {
                    bits.push((*byte >> i) & 1 == 1)
                }
                if IS_PREIMAGE_BE {
                    bits.reverse();
                }
                bits
            })
            .flatten()
            .enumerate()
            .map(|(i, bit)| {
                let allocated_bit =
                    AllocatedBit::alloc(cs.namespace(|| format!("{}", i)), Some(bit))
                        .expect("alloc bit");
                Boolean::from(allocated_bit)
            })
            .collect();

        let digest_bits = sha256(cs.namespace(|| "sha256"), &preimage_bits).unwrap();

        let digest_bytes: Vec<_> = digest_bits
            .chunks(8)
            .map(|bits| {
                let mut byte = 0 as u8;
                let bits = if IS_DIGEST_BE {
                    Vec::from(bits)
                } else {
                    let mut tmp = Vec::from(bits);
                    tmp.reverse();
                    tmp
                };
                for bit in bits {
                    byte = byte * 2
                        + if bit.get_value().expect("value") {
                            1
                        } else {
                            0
                        };
                }
                byte
            })
            .collect();

        assert_eq!(expected_digest, digest_bytes);
    }
}
