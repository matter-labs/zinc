/*
use jab::prelude::*;

fn main() {

    // prove that we know an sha256 preimage
    let xor: BellmanCircuit = circuit! {
        inputs { 
            hash: uint253;
        };

        witness {
            preimage: uint253;
        };

        let hash_bits: memory_vector<bool> = sha256(preimage);
        let truncated: memory_vector<bool> = hash_bits.take(253);
        let output: uint253 = truncated.pack();
        require(hash == output);
    }?;

}
*/


use crate::gadgets::*;
use bellman::pairing::ff::BitIterator;
use bellman::{Circuit, ConstraintSystem, LinearCombination, SynthesisError};
use ff::{Field, PrimeField};
use franklin_crypto::circuit::baby_eddsa::EddsaSignature;
use franklin_crypto::circuit::boolean;
use franklin_crypto::circuit::boolean::{AllocatedBit, Boolean};
use franklin_crypto::circuit::ecc;
use franklin_crypto::circuit::float_point::parse_with_exponent_le;
use franklin_crypto::circuit::num::{AllocatedNum, Num};
use franklin_crypto::circuit::pedersen_hash;
use franklin_crypto::circuit::polynomial_lookup::{do_the_lookup, generate_powers};
use franklin_crypto::circuit::Assignment;
use franklin_crypto::circuit::sha256;
use franklin_crypto::jubjub::{FixedGenerators, JubjubEngine, JubjubParams};
use pairing::bn256::{Bn256, Fr};
use pairing::Engine;

pub struct PreimageCircuit {
    //public input
    pub hash: Option<Fr>,

    //witness input
    pub preimage: Option<Fr>,
}

impl Circuit<Bn256> for PreimageCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let hash = AllocatedNum::alloc(cs.namespace(||"alloc hash field element"),||Ok(self.hash.grab()?))?;
        hash.inputize(cs.namespace(|| "inputize hash"))?;
        let _ = into_bits_le_fixed(cs.namespace(|| "take 253 hash_bits and enforce correctness"), &hash, 253)?; //sometimes we just don't need bits but they are obtained as a side effect of bit_length enforcing

        let preimage = AllocatedNum::alloc(cs.namespace(||"alloc preimage field element"),||Ok(self.preimage.grab()?))?;
        let preimage_bits = into_bits_le_fixed(cs.namespace(|| "take 253 hash_bits and enforce correctness"), &preimage, 253)?;

        let hash_preimage = sha256::sha256(
                cs.namespace(|| "initial rolling sha256"),
                &preimage_bits,
            )?;
        let mut truncated_hash_preimage = hash_preimage.clone();
        truncated_hash_preimage.truncate(253);
        let packed_truncated_hash_preimage = pack_bits_to_element(cs.namespace(||"pack truncated_hash_preimage"), &truncated_hash_preimage)?;
        cs.enforce(
            || "require hash == packed_truncated_hash_preimage",
            |lc|lc + hash.get_variable() ,
            |lc| lc + CS::one(),
            |lc| lc + packed_truncated_hash_preimage.get_variable(),
        );
        Ok(())
    }
}
