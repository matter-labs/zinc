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
use zrust_bytecode::Push;
use zrust_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn sum(a: u8, b: u8) -> u8 {
    let result = a + b;
    result
}

fn main() {
    let result = sum(42, 25);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(7, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Copy(Copy::new(1)),
        Instruction::Copy(Copy::new(0)),
        Instruction::Add(Add),
        Instruction::Copy(Copy::new(2)),
        Instruction::Return(Return::new(1)),
        Instruction::Push(Push::new(BigInt::from(42), false, 8)),
        Instruction::Push(Push::new(BigInt::from(25), false, 8)),
        Instruction::Call(Call::new(2, 2)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
