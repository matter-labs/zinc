//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::data::types::DataType;
use zinc_bytecode::data::types::IntegerType;
use zinc_bytecode::data::types::ScalarType;
use zinc_bytecode::Call;
use zinc_bytecode::Dbg;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

#[test]
fn test() {
    let input = r#"
enum Data {
    VALUE = 42,
}

impl Data {
    const ANOTHER: u8 = 25;

    fn test() {
        dbg!("{}", 42);
    }
}

fn main() {
    Data::test();
    dbg!("{} {}", Data::VALUE, Data::ANOTHER);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(12, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Dbg(Dbg::new(
            "{}".to_owned(),
            vec![DataType::Scalar(ScalarType::Integer(IntegerType {
                is_signed: false,
                bit_length: crate::BITLENGTH_BYTE,
            }))],
        )),
        Instruction::Return(Return::new(0)),
        Instruction::Call(Call::new(2, 0)),
        Instruction::PushConst(PushConst::new(
            BigInt::from(42),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::PushConst(PushConst::new(
            BigInt::from(25),
            false,
            crate::BITLENGTH_BYTE,
        )),
        Instruction::Dbg(Dbg::new(
            "{} {}".to_owned(),
            vec![
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bit_length: crate::BITLENGTH_BYTE,
                })),
                DataType::Scalar(ScalarType::Integer(IntegerType {
                    is_signed: false,
                    bit_length: crate::BITLENGTH_BYTE,
                })),
            ],
        )),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
