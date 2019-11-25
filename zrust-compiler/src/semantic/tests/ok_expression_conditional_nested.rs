//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Call;
use zrust_bytecode::ConditionalSelect;
use zrust_bytecode::Copy;
use zrust_bytecode::Exit;
use zrust_bytecode::FrameBegin;
use zrust_bytecode::FrameEnd;
use zrust_bytecode::Instruction;
use zrust_bytecode::Not;
use zrust_bytecode::PopCondition;
use zrust_bytecode::Push;
use zrust_bytecode::PushCondition;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = if false {
        1
    } else if true {
        if true {
            42
        } else {
            2
        }
    } else {
        3
    };
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(0), false, 1)),
        Instruction::Copy(Copy::new(0)),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(1), false, 1)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(0)),
        Instruction::Not(Not),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(1), false, 1)),
        Instruction::Copy(Copy::new(2)),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(1), false, 1)),
        Instruction::Copy(Copy::new(3)),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(42), false, 8)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(3)),
        Instruction::Not(Not),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(5)),
        Instruction::Copy(Copy::new(4)),
        Instruction::Copy(Copy::new(3)),
        Instruction::ConditionalSelect(ConditionalSelect),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(2)),
        Instruction::Not(Not),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(3), false, 8)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(4)),
        Instruction::Copy(Copy::new(3)),
        Instruction::Copy(Copy::new(2)),
        Instruction::ConditionalSelect(ConditionalSelect),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(2)),
        Instruction::Copy(Copy::new(1)),
        Instruction::Copy(Copy::new(0)),
        Instruction::ConditionalSelect(ConditionalSelect),
        Instruction::FrameEnd(FrameEnd::new(1)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
