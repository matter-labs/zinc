#![allow(unused_variables)]

use r1cs::Bn256;
use r1cs::Circuit;
use r1cs::ConstraintSystem;
use r1cs::Fr;
use r1cs::SynthesisError;

#[derive(Default)]
pub struct GeneratedCircuit {
    pub result: Fr,
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(
        self,
        system: &mut CS,
    ) -> Result<(), SynthesisError> {
        Ok(())
    }
}
