use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use franklin_crypto::circuit::boolean::Boolean;
use franklin_crypto::circuit::num::AllocatedNum;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;

#[derive(Default)]
pub struct GeneratedCircuit {}

impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let a = jab::allocate_input(&mut cs, || Ok(Fr::zero()), "a", 8)?.0;
        let x = jab::allocate_input(&mut cs, || Ok(Fr::zero()), "x", 8)?.0;

        let b = jab::allocate_witness(&mut cs, || Ok(Fr::zero()), "b", 8)?.0;

        let temp_id_000001 = jab::allocate(&mut cs, "temp_id_000001", "2")?;
        let temp_id_000002 = jab::multiplication(&mut cs, &temp_id_000001, &a, "temp_id_000002", 254)?.0;
        let temp_id_000003 = jab::allocate(&mut cs, "temp_id_000003", "15")?;
        let temp_id_000004 = jab::addition(&mut cs, &b, &temp_id_000003, "temp_id_000004", 254)?.0;
        let temp_id_000005 = jab::allocate(&mut cs, "temp_id_000005", "64")?;
        let temp_id_000006 = jab::subtraction(&mut cs, &temp_id_000004, &temp_id_000005, "temp_id_000006", 254)?.0;
        let temp_id_000007 = jab::allocate(&mut cs, "temp_id_000007", "3")?;
        let temp_id_000008 = jab::multiplication(&mut cs, &temp_id_000007, &temp_id_000006, "temp_id_000008", 254)?.0;
        let temp_id_000009 = jab::addition(&mut cs, &temp_id_000002, &temp_id_000008, "temp_id_000009", 254)?.0;
        let temp_id_000010 = jab::allocate(&mut cs, "temp_id_000010", "25")?;
        let temp_id_000011 = jab::multiplication(&mut cs, &temp_id_000010, &x, "temp_id_000011", 254)?.0;
        let temp_id_000012 = jab::subtraction(&mut cs, &temp_id_000009, &temp_id_000011, "temp_id_000012", 254)?.0;
        let temp_id_000013 = {
            let temp_id_000014 = jab::allocate(&mut cs, "temp_id_000014", "2")?;
            let temp_id_000015 = {
                let temp_id_000016 = jab::allocate(&mut cs, "temp_id_000016", "5")?;
                let temp_id_000017 = jab::allocate(&mut cs, "temp_id_000017", "4")?;
                let temp_id_000018 = jab::addition(&mut cs, &temp_id_000016, &temp_id_000017, "temp_id_000018", 254)?.0;
                temp_id_000018
            };
            let temp_id_000019 = jab::addition(&mut cs, &temp_id_000014, &temp_id_000015, "temp_id_000019", 254)?.0;
            temp_id_000019
        };
        let temp_id_000020 = jab::addition(&mut cs, &temp_id_000012, &temp_id_000013, "temp_id_000020", 254)?.0;
        let c = temp_id_000020;
        let temp_id_000021 = jab::allocate(&mut cs, "temp_id_000021", "136")?;
        let d = temp_id_000021;
        let temp_id_000022 = jab::negation(&mut cs, &d, "temp_id_000022", 254)?.0;
        let temp_id_000023 = jab::equals(&mut cs, &c, &temp_id_000022, "temp_id_000023", 254)?;
        let cond = temp_id_000023;
        dbg!(c.get_variable());
        let temp_id_000024 = Boolean::constant(true);
        let temp_id_000025 = jab::or(&mut cs, &cond, &temp_id_000024, "temp_id_000025")?;
        jab::require(&mut cs, &temp_id_000025, "require_test");
        for i_index in 0..10 {
            let iter_name = format!("temp_id_000026_{}", i_index);
            let i = AllocatedNum::alloc(cs.namespace(|| iter_name), || Ok(Fr::from_str(i_index.to_string().as_str()).unwrap()))?;
            for j_index in 0..10 {
                let iter_name = format!("temp_id_000027_{}_{}", i_index, j_index);
                let j = AllocatedNum::alloc(cs.namespace(|| iter_name), || Ok(Fr::from_str(j_index.to_string().as_str()).unwrap()))?;
                dbg!(i.get_variable());
                dbg!(j.get_variable());
            }
        }
        Ok(())
    }
}
