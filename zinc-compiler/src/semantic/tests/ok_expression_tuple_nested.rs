//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::{IntegerType, ScalarType};
use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::StoreByIndex;
use zinc_bytecode::StoreSequence;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut tuple_nested: ((u8, u8), (u8, u8)) = (
        (1, 2),
        (3, 4),
    );

    tuple_nested.0.0 = 42;
    tuple_nested.1.1 = 111;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(2), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(3), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(4), IntegerType::U8.into())),
        Instruction::StoreSequence(StoreSequence::new(0, 4)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(42), IntegerType::U8.into())),
        Instruction::StoreByIndex(StoreByIndex::new(0, 4)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(1), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(111), IntegerType::U8.into())),
        Instruction::StoreByIndex(StoreByIndex::new(0, 4)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}
