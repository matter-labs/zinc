//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Dbg;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoadByIndex;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
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
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(3),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(4),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(5),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreSequence(StoreSequence::new(0, 5)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(5)),
        Instruction::LoopBegin(LoopBegin::new(5)),
        Instruction::Load(Load::new(5)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::Mul(Mul),
        Instruction::LoadByIndex(LoadByIndex::new(0, 5)),
        Instruction::Log(Dbg::new("{}".to_owned(), 1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Load(Load::new(5)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(5)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
