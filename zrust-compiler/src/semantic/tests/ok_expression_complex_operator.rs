//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zrust_bytecode::Add;
use zrust_bytecode::And;
use zrust_bytecode::Call;
use zrust_bytecode::Eq;
use zrust_bytecode::Exit;
use zrust_bytecode::Instruction;
use zrust_bytecode::Mul;
use zrust_bytecode::Or;
use zrust_bytecode::Push;
use zrust_bytecode::Return;
use zrust_bytecode::Sub;
use zrust_bytecode::Xor;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let result = 2 + 2 * 2 - (42 - 7 * 3) == 6 - 21 && (false ^^ (true || (2 + 2 == 5)));
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Mul(Mul),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(3), false, 8)),
        Instruction::Push(Push::new(BigInt::from(7), false, 8)),
        Instruction::Mul(Mul),
        Instruction::Push(Push::new(BigInt::from(42), false, 8)),
        Instruction::Sub(Sub),
        Instruction::Sub(Sub),
        Instruction::Push(Push::new(BigInt::from(21), false, 8)),
        Instruction::Push(Push::new(BigInt::from(6), false, 8)),
        Instruction::Sub(Sub),
        Instruction::Eq(Eq),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Push(Push::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::Push(Push::new(BigInt::from(5), false, 8)),
        Instruction::Eq(Eq),
        Instruction::Push(Push::new(BigInt::from(1), false, 1)),
        Instruction::Or(Or),
        Instruction::Push(Push::new(BigInt::from(0), false, 1)),
        Instruction::Xor(Xor),
        Instruction::And(And),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect("Syntax error"),
    );

    assert_eq!(expected, result);
}
