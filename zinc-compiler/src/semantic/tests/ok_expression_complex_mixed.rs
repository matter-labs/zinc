//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::LoadPush;
use zinc_bytecode::PopStore;
use zinc_bytecode::PopStoreArray;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::BinaryAnalyzer;
use crate::syntax::Parser;

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
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), false, 8)),
        Instruction::PopStoreArray(PopStoreArray::new(0, 4)),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), false, 8)),
        Instruction::PopStoreArray(PopStoreArray::new(4, 4)),
        Instruction::PushConst(PushConst::new(BigInt::from(34), false, 8)),
        Instruction::PopStore(PopStore::new(8)),
        Instruction::LoadPush(LoadPush::new(7)),
        Instruction::LoadPush(LoadPush::new(3)),
        Instruction::Add(Add),
        Instruction::LoadPush(LoadPush::new(8)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(9)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = BinaryAnalyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
