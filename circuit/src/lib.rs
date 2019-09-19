#![allow(unused_imports)]

use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use franklin_crypto::circuit::boolean::Boolean;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

#[derive(Default)]
pub struct GeneratedCircuit {
    pub result: Fr,
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let result = jab::witness_allocation(&mut cs, || Ok(self.result), "result", 254)?.0;
        let temp_000001 = jab::allocation(&mut cs, "temp_000001", "1")?;
        let temp_000002 = {
            let temp_000003 = jab::allocation(&mut cs, "temp_000003", "2")?;
            let temp_000004 = {
                let temp_000005 = jab::allocation(&mut cs, "temp_000005", "3")?;
                let temp_000006 = {
                    let temp_000007 = jab::allocation(&mut cs, "temp_000007", "4")?;
                    temp_000007
                };
                let temp_000008 = jab::addition(&mut cs, &temp_000005, &temp_000006, "temp_000008", 254)?.0;
                let temp_000009 = jab::allocation(&mut cs, "temp_000009", "3")?;
                let temp_000010 = jab::addition(&mut cs, &temp_000008, &temp_000009, "temp_000010", 254)?.0;
                temp_000010
            };
            let temp_000011 = jab::addition(&mut cs, &temp_000003, &temp_000004, "temp_000011", 254)?.0;
            let temp_000012 = jab::allocation(&mut cs, "temp_000012", "2")?;
            let temp_000013 = jab::addition(&mut cs, &temp_000011, &temp_000012, "temp_000013", 254)?.0;
            temp_000013
        };
        let temp_000014 = jab::addition(&mut cs, &temp_000001, &temp_000002, "temp_000014", 254)?.0;
        let temp_000015 = jab::allocation(&mut cs, "temp_000015", "1")?;
        let temp_000016 = jab::addition(&mut cs, &temp_000014, &temp_000015, "temp_000016", 254)?.0;
        let pyramid = temp_000016.clone();
        let temp_000017 = jab::equals(&mut cs, &pyramid, &result, "temp_000017", 254)?;
        jab::require(&mut cs, &temp_000017, "19_1");
        Ok(())
    }
}
