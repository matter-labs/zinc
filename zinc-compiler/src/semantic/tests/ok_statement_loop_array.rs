//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::data::types::DataType;
use zinc_bytecode::data::types::IntegerType;
use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Dbg;
use zinc_bytecode::EndIf;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoadByIndex;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::Lt;
use zinc_bytecode::Mul;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;
use zinc_bytecode::StoreSequence;

#[test]
fn test() {
    let input = r#"
fn main() {
    let array = [1, 2, 3, 4, 5];
    for i in 0..5 {
        dbg!("{}", array[i]);
    }
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
        Instruction::StoreSequence(StoreSequence::new(0, 5)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), IntegerType::U8.into())),
        Instruction::Store(Store::new(5)),
        Instruction::LoopBegin(LoopBegin::new(5)),
        Instruction::Load(Load::new(5)),
        Instruction::Cast(Cast::new(ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), ScalarType::Field)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), ScalarType::Field)),
        Instruction::Add(Add),
        Instruction::Mul(Mul),
        Instruction::LoadByIndex(LoadByIndex::new(0, 5)),
        Instruction::Dbg(Dbg::new(
            "{}".to_owned(),
            vec![DataType::Scalar(ScalarType::Integer(IntegerType {
                signed: false,
                length: crate::BITLENGTH_BYTE,
            }))],
        )),
        Instruction::Load(Load::new(5)),
        Instruction::PushConst(PushConst::new(BigInt::from(255), IntegerType::U8.into())),
        Instruction::Lt(Lt),
        Instruction::If(If),
        Instruction::Load(Load::new(5)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Add(Add),
        Instruction::Store(Store::new(5)),
        Instruction::EndIf(EndIf),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}
