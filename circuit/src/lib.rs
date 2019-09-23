#![allow(unused_imports)]

use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use franklin_crypto::circuit::boolean::Boolean;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

#[derive(Default)]
pub struct GeneratedCircuit {
    pub dummy: Fr,
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let dummy = jab::input_allocation(&mut cs, || Ok(self.dummy), "dummy", 254)?.0;
        let temp_000001 = jab::allocation(&mut cs, "temp_000001", "5")?;
        let mut result = temp_000001;
        let temp_000002 = Boolean::constant(false);
        let temp_000003 = {
            let temp_000004 = jab::allocation(&mut cs, "temp_000004", "10")?;
            if temp_000002.get_value().unwrap() {
                result = temp_000004;
            }
        };
        dbg!(result.get_value());
        let temp_000005 = jab::allocation(&mut cs, "temp_000005", "5")?;
        let temp_000006 = jab::equals(&mut cs, &result, &temp_000005, "temp_000006", 254)?;
        jab::require(&mut cs, &temp_000006, "14_1");
        Ok(())
    }
}
