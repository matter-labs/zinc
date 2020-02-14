//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::Call;
use zinc_bytecode::EndIf;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

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
        Instruction::PushConst(PushConst::new(BigInt::from(5), IntegerType::U8.into())),
        Instruction::Store(Store::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(0), IntegerType::BOOLEAN.into())),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(10), IntegerType::U8.into())),
        Instruction::Store(Store::new(0)),
        Instruction::EndIf(EndIf),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}
