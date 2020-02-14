//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::scalar::IntegerType;
use zinc_bytecode::Call;
use zinc_bytecode::Else;
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
        Instruction::PushConst(PushConst::new(BigInt::from(0), IntegerType::BOOLEAN.into())),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::U8.into())),
        Instruction::Else(Else),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::BOOLEAN.into())),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(1), IntegerType::BOOLEAN.into())),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(BigInt::from(42), IntegerType::U8.into())),
        Instruction::Else(Else),
        Instruction::PushConst(PushConst::new(BigInt::from(2), IntegerType::U8.into())),
        Instruction::EndIf(EndIf),
        Instruction::Else(Else),
        Instruction::PushConst(PushConst::new(BigInt::from(3), IntegerType::U8.into())),
        Instruction::EndIf(EndIf),
        Instruction::EndIf(EndIf),
        Instruction::Store(Store::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(result, expected);
}