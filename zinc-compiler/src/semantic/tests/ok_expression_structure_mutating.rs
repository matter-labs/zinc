//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::StoreByIndex;
use zinc_bytecode::StoreSequence;

#[test]
fn test() {
    let input = r#"
struct Test {
    x: u8,
    y: u8,
    z: u8,
}

fn main() {
    let mut test = struct Test {
        x: 1,
        y: 2,
        z: 3,
    };

    test.x = 5;
    test.y = 7;
    test.z = 9;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(3),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreSequence(StoreSequence::new(0, 3)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(5),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreByIndex(StoreByIndex::new(0, 3)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(7),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreByIndex(StoreByIndex::new(0, 3)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(9),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreByIndex(StoreByIndex::new(0, 3)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
