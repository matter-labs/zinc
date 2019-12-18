//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;
use zinc_bytecode::StoreArray;

#[test]
fn test() {
    let input = r#"
type Tuple = (u8, u8);

fn main() {
    let tuple: Tuple = (11, 42);
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(11), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(42), false, 8)),
        Instruction::StoreArray(StoreArray::new(0, 2)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
