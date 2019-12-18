//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Assert;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Eq;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut value_1: field = 0;
    let mut value_2: field = 1;
    let mut fibo = value_1;

    for i in 1..=10 {
        fibo = value_1 + value_2;
        value_1 = value_2;
        value_2 = fibo;
    }

    assert!(fibo == 89 as field);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), false, 8)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::Store(Store::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::Store(Store::new(1)),
        Instruction::Load(Load::new(0)),
        Instruction::Store(Store::new(2)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Store(Store::new(3)),
        Instruction::LoopBegin(LoopBegin::new(10)),
        Instruction::Load(Load::new(1)),
        Instruction::Load(Load::new(0)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(2)),
        Instruction::Load(Load::new(1)),
        Instruction::Store(Store::new(0)),
        Instruction::Load(Load::new(2)),
        Instruction::Store(Store::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Load(Load::new(3)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(3)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::PushConst(PushConst::new(BigInt::from(89), false, 8)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::Load(Load::new(2)),
        Instruction::Eq(Eq),
        Instruction::Assert(Assert),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
