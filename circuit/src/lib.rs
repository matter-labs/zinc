use bellman::Circuit;
use bellman::ConstraintSystem;
use bellman::SynthesisError;
use ff::Field;
use ff::PrimeField;
use pairing::bn256::Bn256;
use pairing::bn256::Fr;
use franklin_crypto::circuit::num::AllocatedNum;

#[derive(Default)]
pub struct GeneratedCircuit {}
impl Circuit<Bn256> for GeneratedCircuit {
    fn synthesize<CS: ConstraintSystem<Bn256>>(self, mut cs: &mut CS) -> Result<(), SynthesisError> {
        let a = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "a", 8)?.0;
        let x = jab::alloc_input(&mut cs, || Ok(Fr::zero()), "x", 8)?.0;

        let b = jab::alloc_witness(&mut cs, || Ok(Fr::zero()), "b", 8)?.0;

        let line_14_11_multiplication_operand_2 = a;
        let line_14_11_multiplication_operand_1 = AllocatedNum::alloc(cs.namespace(|| "line_14_11_multiplication_operand_1"), || Ok(Fr::from_str("2").unwrap()))?;
        let line_14_11_multiplication_result = jab::mul(&mut cs, &line_14_11_multiplication_operand_1, &line_14_11_multiplication_operand_2, "line_14_11_multiplication_result", 254)?.0;
        let line_14_24_addition_operand_2 = AllocatedNum::alloc(cs.namespace(|| "line_14_24_addition_operand_2"), || Ok(Fr::from_str("15").unwrap()))?;
        let line_14_24_addition_operand_1 = b;
        let line_14_24_addition_result = jab::sum(&mut cs, &line_14_24_addition_operand_1, &line_14_24_addition_operand_2, "line_14_24_addition_result", 254)?.0;
        let line_14_29_subtraction_operand_2 = AllocatedNum::alloc(cs.namespace(|| "line_14_29_subtraction_operand_2"), || Ok(Fr::from_str("64").unwrap()))?;
        let line_14_29_subtraction_operand_1 = line_14_24_addition_result;
        let line_14_29_subtraction_result = jab::diff(&mut cs, &line_14_29_subtraction_operand_1, &line_14_29_subtraction_operand_2, "line_14_29_subtraction_result", 254)?.0;
        let line_14_19_multiplication_operand_2 = line_14_29_subtraction_result;
        let line_14_19_multiplication_operand_1 = AllocatedNum::alloc(cs.namespace(|| "line_14_19_multiplication_operand_1"), || Ok(Fr::from_str("3").unwrap()))?;
        let line_14_19_multiplication_result = jab::mul(&mut cs, &line_14_19_multiplication_operand_1, &line_14_19_multiplication_operand_2, "line_14_19_multiplication_result", 254)?.0;
        let line_14_15_addition_operand_2 = line_14_19_multiplication_result;
        let line_14_15_addition_operand_1 = line_14_11_multiplication_result;
        let line_14_15_addition_result = jab::sum(&mut cs, &line_14_15_addition_operand_1, &line_14_15_addition_operand_2, "line_14_15_addition_result", 254)?.0;
        let line_14_40_multiplication_operand_2 = x;
        let line_14_40_multiplication_operand_1 = AllocatedNum::alloc(cs.namespace(|| "line_14_40_multiplication_operand_1"), || Ok(Fr::from_str("25").unwrap()))?;
        let line_14_40_multiplication_result = jab::mul(&mut cs, &line_14_40_multiplication_operand_1, &line_14_40_multiplication_operand_2, "line_14_40_multiplication_result", 254)?.0;
        let line_14_35_subtraction_operand_2 = line_14_40_multiplication_result;
        let line_14_35_subtraction_operand_1 = line_14_15_addition_result;
        let line_14_35_subtraction_result = jab::diff(&mut cs, &line_14_35_subtraction_operand_1, &line_14_35_subtraction_operand_2, "line_14_35_subtraction_result", 254)?.0;
        let c = line_14_35_subtraction_result;
        let line_15_3_addition_operand_2 = AllocatedNum::alloc(cs.namespace(|| "line_15_3_addition_operand_2"), || Ok(Fr::from_str("2").unwrap()))?;
        let line_15_3_addition_operand_1 = AllocatedNum::alloc(cs.namespace(|| "line_15_3_addition_operand_1"), || Ok(Fr::from_str("2").unwrap()))?;
        let line_15_3_addition_result = jab::sum(&mut cs, &line_15_3_addition_operand_1, &line_15_3_addition_operand_2, "line_15_3_addition_result", 254)?.0;
        let expression_000001_result = line_15_3_addition_result;
        dbg!(c.get_variable());
        let expression_000002_result = c;
        jab::require(&mut cs, expression_000002_result, "require_test");

        Ok(())
    }
}
