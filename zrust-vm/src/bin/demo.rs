use franklin_crypto::circuit::test::TestConstraintSystem;
use zrust_vm::{VirtualMachine, Bytecode, OpCode};
use bellman::pairing::bn256::{Bn256, Fr};

fn main() {
    let mut cs = TestConstraintSystem::<Bn256>::new();
    let mut vm = VirtualMachine::<Bn256, TestConstraintSystem<Bn256>>::new();

    let mut bytecode = Bytecode::new(&[
        OpCode::Push as u8, 0x01, 0xAA,
        OpCode::Push as u8, 0x02, 0xBB, 0xBB,
        OpCode::Pop as u8,
        OpCode::Push as u8, 0x01, 0x55,
        OpCode::Add as u8,
    ]);

    vm.run(&mut cs, &mut bytecode).expect("failed to execute bytecode");

    let top = vm
        .stack()
        .top()
        .unwrap()
        .value
        .unwrap();

    let expected = Fr::from_hex("0xFF").unwrap();

    assert_eq!(top, expected, "sum is not correct");

    println!("Ok!");
}
