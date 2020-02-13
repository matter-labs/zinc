//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::{IntegerType, ScalarType};
use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Mul;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::StoreByIndex;
use zinc_bytecode::StoreSequence;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut array_double: [[u8; 4]; 4] = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];

    array_double[3][1] = 111;
    array_double[1][3] = 42;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(2), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(3), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(4), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(5), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(6), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(7), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(8), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(9), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(10), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(11), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(12), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(13), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(14), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(15), IntegerType::U8.into())),
        Instruction::PushConst(PushConst::new(BigInt::from(16), IntegerType::U8.into())),
        Instruction::StoreSequence(StoreSequence::new(0, 16)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::Mul(Mul),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), ScalarType::Field)),
        Instruction::Mul(Mul),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(111), IntegerType::U8.into())),
        Instruction::StoreByIndex(StoreByIndex::new(0, 16)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::Mul(Mul),
        Instruction::PushConst(PushConst::new(BigInt::from(3), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), ScalarType::Field)),
        Instruction::Mul(Mul),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(42), IntegerType::U8.into())),
        Instruction::StoreByIndex(StoreByIndex::new(0, 16)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}
