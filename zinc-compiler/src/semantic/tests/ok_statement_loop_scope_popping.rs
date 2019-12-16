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
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

#[test]
fn test() {
    let input = r#"
fn main() {
    for i in 0..5 {}

    for i in 5..10 {}
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::LoopBegin(LoopBegin::new(5)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::PushConst(PushConst::new(BigInt::from(5), false, 8)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoopBegin(LoopBegin::new(5)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
