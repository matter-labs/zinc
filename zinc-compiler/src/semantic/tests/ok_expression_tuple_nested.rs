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
fn main() {
    let mut tuple_nested: ((u8, u8), (u8, u8)) = (
        (1, 2),
        (3, 4),
    );

    tuple_nested.0.0 = 42;
    tuple_nested.1.1 = 111;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), false, 8)),
        Instruction::PopStoreArray(PopStoreArray::new(0, 4)),
        Instruction::PushConst(PushConst::new(BigInt::from(42), false, 8)),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(111), false, 8)),
        Instruction::PopStore(PopStore::new(3)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
