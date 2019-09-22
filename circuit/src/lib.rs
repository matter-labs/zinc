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
        let temp_000001 = Boolean::constant(false);
        let temp_000002 = {
            let temp_000003 = jab::allocation(&mut cs, "temp_000003", "1")?;
            temp_000003
        };
        let temp_000004 = Boolean::constant(true);
        let temp_000005 = {
            let temp_000006 = jab::allocation(&mut cs, "temp_000006", "2")?;
            temp_000006
        };
        let temp_000007 = {
            let temp_000008 = jab::allocation(&mut cs, "temp_000008", "3")?;
            temp_000008
        };
        let temp_000009 = jab::conditional(&mut cs, &temp_000005, &temp_000007, &temp_000004, "temp_000009")?;
        let temp_000010 = jab::conditional(&mut cs, &temp_000002, &temp_000009, &temp_000001, "temp_000010")?;
        let result = temp_000010.clone();
        dbg!(result.get_value());
        let temp_000011 = jab::allocation(&mut cs, "temp_000011", "0")?;
        let temp_000012 = jab::equals(&mut cs, &dummy, &temp_000011, "temp_000012", 254)?;
        jab::require(&mut cs, &temp_000012, "17_1");
        Ok(())
    }
}
