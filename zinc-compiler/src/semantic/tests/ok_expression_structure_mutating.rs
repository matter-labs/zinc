//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PopStore;
use zinc_bytecode::PopStoreArray;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

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
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::PopStoreArray(PopStoreArray::new(0, 3)),
        Instruction::PushConst(PushConst::new(BigInt::from(5), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(7), false, 8)),
        Instruction::PopStore(PopStore::new(1)),
        Instruction::PushConst(PushConst::new(BigInt::from(9), false, 8)),
        Instruction::PopStore(PopStore::new(2)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
