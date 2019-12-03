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
use zinc_bytecode::LoadPush;
use zinc_bytecode::LoopBegin;
use zinc_bytecode::LoopEnd;
use zinc_bytecode::Mul;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

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
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoopBegin(LoopBegin::new(4)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Mul(Mul),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::LoadPush(LoadPush::new(1)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::LoopEnd(LoopEnd),
        Instruction::PushConst(PushConst::new(BigInt::from(120), false, 8)),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Eq(Eq),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::Assert(Assert),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
