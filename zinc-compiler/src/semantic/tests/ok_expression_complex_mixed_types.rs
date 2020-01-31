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
use zinc_bytecode::LoadByIndex;
use zinc_bytecode::Mul;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;
use zinc_bytecode::StoreSequence;

#[test]
fn test() {
    let input = r#"
struct Data { value: u8 }

fn main() {
    let array = [[1, 2], [3, 4]];
    let tuple = ((1, 2), (3, 4));
    let structure = struct Data { value: 34 };

    let result = array[1][1] + tuple.1.1 + structure.value;
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
        Instruction::PushConst(PushConst::new(
            BigInt::from(4),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreSequence(StoreSequence::new(0, 4)),
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
        Instruction::PushConst(PushConst::new(
            BigInt::from(4),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::StoreSequence(StoreSequence::new(4, 4)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(34),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Store(Store::new(8)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::Mul(Mul),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Cast(Cast::new(false, crate::BITLENGTH_FIELD)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Mul(Mul),
        Instruction::Add(Add),
        Instruction::LoadByIndex(LoadByIndex::new(0, 4)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::LoadByIndex(LoadByIndex::new(4, 4)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_FIELD,
        )),
        Instruction::Add(Add),
        Instruction::LoadByIndex(LoadByIndex::new(8, 1)),
        Instruction::Add(Add),
        Instruction::Store(Store::new(9)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
