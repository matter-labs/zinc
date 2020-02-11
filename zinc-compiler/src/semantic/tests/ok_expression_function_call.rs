//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
fn sum(a: u8, b: u8) -> u8 {
    let result = a + b;
    result
}

fn main() {
    let result = sum(42, 25);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(18, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Load(Load::new(0)),
        Instruction::Load(Load::new(1)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(2)),
        Instruction::Load(Load::new(2)),
        Instruction::Return(Return::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(25),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Call(Call::new(2, 2)),
        Instruction::Store(Store::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
