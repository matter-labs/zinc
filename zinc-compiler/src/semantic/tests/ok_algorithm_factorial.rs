//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::{IntegerType, ScalarType};
use zinc_bytecode::Add;
use zinc_bytecode::Assert;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::EndIf;
use zinc_bytecode::Eq;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::Lt;
use zinc_bytecode::Mul;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut fact: field = 1;

    for i in 2..6 {
        fact = fact * i as field;
    }

    assert!(fact == 120 as field);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::Store(Store::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), IntegerType::U8.into())),
        Instruction::Store(Store::new(1)),
        Instruction::LoopBegin(LoopBegin::new(4)),
        Instruction::Load(Load::new(1)),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::Load(Load::new(0)),
        Instruction::Mul(Mul),
        Instruction::Store(Store::new(0)),
        Instruction::Load(Load::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(255), IntegerType::U8.into())),
        Instruction::Lt(Lt),
        Instruction::If(If),
        Instruction::Load(Load::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Add(Add),
        Instruction::Store(Store::new(1)),
        Instruction::EndIf(EndIf),
        Instruction::LoopEnd(LoopEnd),
        Instruction::PushConst(PushConst::new(BigInt::from(120), IntegerType::U8.into())),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::Load(Load::new(0)),
        Instruction::Eq(Eq),
        Instruction::Assert(Assert::new(None)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}
