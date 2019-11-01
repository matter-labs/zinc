mod opcodes;
mod stack;
mod vm;
mod bytecode;
mod operator;

pub mod operators;

pub use opcodes::OpCode;
pub use stack::Stack;
pub use vm::{VirtualMachine, RuntimeError};
pub use bytecode::Bytecode;
pub use operator::Operator;


#[cfg(test)]
mod test {
    use super::*;
    use franklin_crypto::circuit::test::TestConstraintSystem;
    use bellman::pairing::bn256::{Bn256, Fr};

    #[test]
    fn test_vm() {
        let mut cs = TestConstraintSystem::<Bn256>::new();
        let mut vm = VirtualMachine::<Bn256, TestConstraintSystem<Bn256>>::new();
        let mut bytecode = Bytecode::new(&[
            OpCode::Push as u8, 0x01, 0xAA,
            OpCode::Push as u8, 0x02, 0xBB, 0xBB,
            OpCode::Pop as u8,
            OpCode::Push as u8, 0x01, 0x55,
            OpCode::Add as u8,
        ]);

        match vm.run(&mut cs, &mut bytecode) {
            Ok(_) => {},
            Err(e) => {assert!(false, "runtime error: {:?}", e)},
        }

        let top = vm
            .stack()
            .top()
            .unwrap()
            .value
            .unwrap();

        let expected = Fr::from_hex("0xFF").unwrap();

        assert_eq!(top, expected, "sum is not correct");
    }
}
