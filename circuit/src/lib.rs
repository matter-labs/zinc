use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

#[derive(Default)]
pub struct GeneratedCircuit {
    pub dummy: Fr,
}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let dummy = jab::allocate_input(cs.namespace(|| "dummy"), || Ok(self.dummy), 254)?.0;
        let temp_000001 = jab::allocate_number(cs.namespace(|| "temp_000001"), "5")?;
        let mut result = temp_000001.clone();
        let temp_000002 = jab::allocate_boolean(cs.namespace(|| "temp_000002"), false)?;
        let temp_000003 = {
            let temp_000004 = jab::allocate_number(cs.namespace(|| "temp_000004"), "10")?;
            if temp_000002.get_value().unwrap() {
                result = temp_000004.clone();
            }
        };
        dbg!(result.get_value());
        let temp_000005 = jab::allocate_number(cs.namespace(|| "temp_000005"), "5")?;
        let temp_000006 =
            jab::equals_number(cs.namespace(|| "temp_000006"), &result, &temp_000005, 254)?;
        jab::require(cs.namespace(|| "14_1"), &temp_000006, "14_1");
        Ok(())
    }
}
