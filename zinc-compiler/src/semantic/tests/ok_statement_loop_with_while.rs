//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

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
use zinc_bytecode::Not;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in 0..=(2+3)*2 while i < 5 {
        sum = sum + i;
    }
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(2)),
        Instruction::LoopBegin(LoopBegin::new(11)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(5),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Load(Load::new(1)),
        Instruction::Lt(Lt),
        Instruction::Not(Not),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(2)),
        Instruction::EndIf(EndIf),
        Instruction::Load(Load::new(2)),
        Instruction::If(If),
        Instruction::Load(Load::new(1)),
        Instruction::Load(Load::new(0)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(0)),
        Instruction::EndIf(EndIf),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Load(Load::new(1)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(1)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
