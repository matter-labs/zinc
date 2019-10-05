#![allow(unused_imports)]
#![allow(unused_variables)]

use r1cs::Bn256;
use r1cs::Circuit;
use r1cs::ConstraintSystem;
use r1cs::Fr;
use r1cs::SynthesisError;

#[derive(Default)]
pub struct GeneratedCircuit {}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(
        self,
        system: &mut CS,
    ) -> Result<(), SynthesisError> {
        let temp_000001 = r1cs::allocate_number(system.namespace(|| "temp_000001"), "1")?;
        let mut fact = temp_000001;
        for i_index in 2..6 {
            let i = r1cs::allocate_number(
                system.namespace(|| format!("temp_000002_{}", i_index)),
                i_index.to_string().as_str(),
            )?;
            let temp_000003 = r1cs::cast(
                system.namespace(|| format!("temp_000003_{}", i_index)),
                &i,
                254,
            )?;
            let temp_000004 = r1cs::multiply(
                system.namespace(|| format!("temp_000004_{}", i_index)),
                &fact,
                &temp_000003,
                254,
            )?
            .0;
            fact = temp_000004;
        }
        let temp_000005 = r1cs::allocate_number(system.namespace(|| "temp_000005"), "120")?;
        let temp_000006 = r1cs::cast(system.namespace(|| "temp_000006"), &temp_000005, 254)?;
        let temp_000007 =
            r1cs::equals_number(system.namespace(|| "temp_000007"), &fact, &temp_000006, 254)?;
        r1cs::require(system.namespace(|| "L13"), &temp_000007, "L13");
        Ok(())
    }
}
