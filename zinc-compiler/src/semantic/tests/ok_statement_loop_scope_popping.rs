//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::EndIf;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::Lt;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
fn main() {
    for i in 0..2+3 {}

    for i in 2+3..(2+3)*2 {}
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), IntegerType::U8.into())),
        Instruction::Store(Store::new(0)),
        Instruction::LoopBegin(LoopBegin::new(5)),
        Instruction::Load(Load::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(255), IntegerType::U8.into())),
        Instruction::Lt(Lt),
        Instruction::If(If),
        Instruction::Load(Load::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Add(Add),
        Instruction::Store(Store::new(0)),
        Instruction::EndIf(EndIf),
        Instruction::LoopEnd(LoopEnd),
        Instruction::PushConst(PushConst::new(BigInt::from(5), IntegerType::U8.into())),
        Instruction::Store(Store::new(1)),
        Instruction::LoopBegin(LoopBegin::new(5)),
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
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}