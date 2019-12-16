//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Call;
use zinc_bytecode::Else;
use zinc_bytecode::EndIf;
use zinc_bytecode::Eq;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::LoadPush;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = 2;
    let result = match value {
        1 => 1,
        2 => 2,
        inner => match inner {
            255 => 0,
            _ => 42,
        },
    };
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Eq(Eq),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Else(Else),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::Eq(Eq),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::Else(Else),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::LoadPush(LoadPush::new(2)),
        Instruction::PushConst(PushConst::new(BigInt::from(255), false, 8)),
        Instruction::Eq(Eq),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::Else(Else),
        Instruction::PushConst(PushConst::new(BigInt::from(42), false, 8)),
        Instruction::EndIf(EndIf),
        Instruction::EndIf(EndIf),
        Instruction::EndIf(EndIf),
        Instruction::PopStore(PopStore::new(3)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
