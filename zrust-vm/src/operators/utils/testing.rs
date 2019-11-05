use franklin_crypto::circuit::test::TestConstraintSystem;
use bellman::pairing::bn256::{Bn256, Fr};
use crate::{VirtualMachine, Bytecode, Stack};

pub fn execute_bytecode(bytecode: &mut Bytecode) -> Stack<Bn256> {
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::<Bn256, TestConstraintSystem<Bn256>>::new();
    vm.run(&mut cs, bytecode).expect("failed to execute bytecode");
    vm.stack().clone()
}

pub fn assert_stack_value(stack: &Stack<Bn256>, index: usize, hex: &str) {
    let primitive = stack
        .get(index)
        .expect(format!("no element at index {}", index).as_str());

    let value = primitive.value.expect("value is None");

    let expected = Fr::from_hex(hex).expect(format!("invalid hex literal: {:?}", hex).as_str());

    assert_eq!(value, expected);
}
