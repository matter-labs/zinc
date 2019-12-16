//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::LoadPush;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

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
        Instruction::Call(Call::new(8, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::LoadPush(LoadPush::new(2)),
        Instruction::Return(Return::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(25), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(42), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::Call(Call::new(2, 2)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
