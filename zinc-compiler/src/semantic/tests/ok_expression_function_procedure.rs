//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

#[test]
fn test() {
    let input = r#"
fn check(a: u8) {}

fn main() {
    check(42);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(7, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::Return(Return::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(42), IntegerType::U8.into())),
        Instruction::Call(Call::new(2, 1)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}