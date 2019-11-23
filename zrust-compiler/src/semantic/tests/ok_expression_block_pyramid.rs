//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Add;
use zrust_bytecode::Call;
use zrust_bytecode::Exit;
use zrust_bytecode::Instruction;
use zrust_bytecode::Push;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let pyramid = 1 + {
        2 + {
            3 + {
                4
            } + 3
        } + 2
    } + 1;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Push(Push::new(BigInt::from(4), false, 8)),
        Instruction::Push(Push::new(BigInt::from(3), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(3), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(1), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(1), false, 8)),
        Instruction::Add(Add),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
