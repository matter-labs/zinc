#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(clippy::let_and_return)]

use r1cs::ConstraintSystem;
use r1cs::Circuit;
use r1cs::SynthesisError;
use r1cs::Bn256;
use r1cs::Fr;
use r1cs::Boolean;
use r1cs::AllocatedNum;

#[derive(Default)]
pub struct GeneratedCircuit {
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<S: ConstraintSystem<Bn256>>(self, system: &mut S) -> Result<(), SynthesisError> {
        let temp_000001 = r1cs::allocate_number(system.namespace(|| "temp_000001"), "0")?;
        let mut sum: AllocatedNum<Bn256> = temp_000001;
        for i_index in (0..=10).rev() {
            let i = r1cs::allocate_number(system.namespace(|| format!("temp_000001_{}", i_index)), i_index.to_string().as_str())?;
            let temp_000002 = r1cs::add(system.namespace(|| format!("temp_000002_{}", i_index)), &sum, &i, 254)?.0;
            sum = temp_000002;
        }
        let temp_000004 = r1cs::allocate_number(system.namespace(|| "temp_000004"), "55")?;
        let temp_000003 = r1cs::cast(system.namespace(|| "temp_000003"), &temp_000004, 254)?;
        let temp_000005 = r1cs::equals_number(system.namespace(|| "temp_000005"), &sum, &temp_000003, 254)?;
        r1cs::require(system.namespace(|| "L12"), &temp_000005, "L12");
        Ok(())
    }
}
