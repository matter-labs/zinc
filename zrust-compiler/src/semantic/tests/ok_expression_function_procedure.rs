//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Call;
use zrust_bytecode::Exit;
use zrust_bytecode::Instruction;
use zrust_bytecode::Push;
use zrust_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn check(a: u8) {}

fn main() {
    check(42);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(3, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Return(Return::new(0)),
        Instruction::Push(Push::new(BigInt::from(42), false, 8)),
        Instruction::Call(Call::new(2, 1)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
