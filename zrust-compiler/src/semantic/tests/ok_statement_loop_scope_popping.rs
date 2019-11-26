//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Add;
use zrust_bytecode::Call;
use zrust_bytecode::Copy;
use zrust_bytecode::Exit;
use zrust_bytecode::Instruction;
use zrust_bytecode::LoopBegin;
use zrust_bytecode::LoopEnd;
use zrust_bytecode::Push;
use zrust_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    for i in 0..5 {}

    for i in 5..10 {}
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Push(Push::new(BigInt::from(0), false, 8)),
        Instruction::LoopBegin(LoopBegin::new(5, 1)),
        Instruction::Push(Push::new(BigInt::from(1), false, 8)),
        Instruction::Copy(Copy::new(0)),
        Instruction::Add(Add),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Push(Push::new(BigInt::from(5), false, 8)),
        Instruction::LoopBegin(LoopBegin::new(5, 1)),
        Instruction::Push(Push::new(BigInt::from(1), false, 8)),
        Instruction::Copy(Copy::new(1)),
        Instruction::Add(Add),
        Instruction::LoopEnd(LoopEnd),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
