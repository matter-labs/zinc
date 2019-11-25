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
use zrust_bytecode::PopCondition;
use zrust_bytecode::Push;
use zrust_bytecode::PushCondition;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let mut result = 5;
    if false {
        result = 10;
    };
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Push(Push::new(BigInt::from(5), false, 8)),
        Instruction::Push(Push::new(BigInt::from(0), false, 1)),
        Instruction::Copy(Copy::new(1)),
        Instruction::PushCondition(PushCondition),
        Instruction::FrameBegin(FrameBegin),
        Instruction::Push(Push::new(BigInt::from(10), false, 8)),
        Instruction::Copy(Copy::new(2)),
        Instruction::FrameEnd(FrameEnd::new(1)),
        Instruction::PopCondition(PopCondition),
        Instruction::Copy(Copy::new(0)),
        Instruction::Copy(Copy::new(2)),
        Instruction::Copy(Copy::new(1)),
        Instruction::ConditionalSelect(ConditionalSelect),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
