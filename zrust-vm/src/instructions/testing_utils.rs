use crate::{VMInstruction, VirtualMachine, Stack, RuntimeError};
use franklin_crypto::circuit::test::TestConstraintSystem;
use bellman::pairing::bn256::{Bn256, Fr};

pub fn create_instructions_vec() -> Vec<Box<dyn VMInstruction<Bn256, TestConstraintSystem<Bn256>>>> {
    Vec::new()
}

pub fn execute(instructions: &[Box<dyn VMInstruction<Bn256, TestConstraintSystem<Bn256>>>])
    -> Result<Stack<Bn256>, RuntimeError>
{
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::<Bn256>::new();
    vm.run(&mut cs, instructions)?;

    Ok(vm.stack().clone())
}

pub fn assert_stack_value(stack: &Stack<Bn256>, index: usize, hex: &str) {
    let primitive = stack
        .get(index)
        .expect(format!("no element at index {}", index).as_str());

    let value = primitive.value.expect("value is None");

    let expected = Fr::from_hex(hex).expect(format!("invalid hex literal: {:?}", hex).as_str());

    assert_eq!(value, expected);
}
