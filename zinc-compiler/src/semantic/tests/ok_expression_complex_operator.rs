//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::And;
use zinc_bytecode::Call;
use zinc_bytecode::Eq;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::Mul;
use zinc_bytecode::Or;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::Store;
use zinc_bytecode::Sub;
use zinc_bytecode::Xor;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let result = 2 + 2 * 2 - (42 - 7 * 3) == { 6 - 21 } && (false ^^ (true || (2 + 2 == 5)));
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Mul(Mul),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(
            BigInt::from(3),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(7),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Mul(Mul),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Sub(Sub),
        Instruction::Sub(Sub),
        Instruction::PushConst(PushConst::new(
            BigInt::from(21),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(6),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Sub(Sub),
        Instruction::Eq(Eq),
        Instruction::PushConst(PushConst::new(
            BigInt::from(2),
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
            BigInt::from(5),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Eq(Eq),
        Instruction::PushConst(PushConst::new(
            BigInt::from(1),
            false,
            crate::BITLENGTH_BOOLEAN,
        )),
        Instruction::Or(Or),
        Instruction::PushConst(PushConst::new(
            BigInt::from(0),
            false,
            crate::BITLENGTH_BOOLEAN,
        )),
        Instruction::Xor(Xor),
        Instruction::And(And),
        Instruction::Store(Store::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
