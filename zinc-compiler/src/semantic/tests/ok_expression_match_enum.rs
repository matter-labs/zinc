//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Call;
use zinc_bytecode::Else;
use zinc_bytecode::EndIf;
use zinc_bytecode::Eq;
use zinc_bytecode::Exit;
use zinc_bytecode::If;
use zinc_bytecode::Instruction;
use zinc_bytecode::Load;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
enum Language {
    JABBERWOCKY = 0,
    ZINC = 42,
    RUST = 255,
}

fn main(input: (), witness: ()) -> u8 {
    let language = Language::ZINC;
    match language {
        Language::ZINC => 1,
        Language::RUST => 1,
        _ => 0,
    }
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(0)),
        Instruction::Load(Load::new(0)),
        Instruction::Store(Store::new(1)),
        Instruction::Load(Load::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Eq(Eq),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Else(Else),
        Instruction::Load(Load::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(255),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Eq(Eq),
        Instruction::If(If),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Else(Else),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::EndIf(EndIf),
        Instruction::EndIf(EndIf),
        Instruction::Return(Return::new(1)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
