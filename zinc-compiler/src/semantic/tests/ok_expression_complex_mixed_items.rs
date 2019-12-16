//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Cast;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::LoadPush;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::BinaryAnalyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
static STATIC: field = 5;

const CONST: field = 42;

fn main() -> field {
    let var: field = 69;

    STATIC + CONST + var
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(4, 0)),
        Instruction::Exit(Exit::new(1)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(5),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(69),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD as u8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Add(Add),
        Instruction::LoadPush(LoadPush::new(0)),
        Instruction::Add(Add),
        Instruction::Return(Return::new(1)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
