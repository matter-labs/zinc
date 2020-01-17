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
use zinc_bytecode::Store;

#[test]
fn test() {
    let input = r#"
enum First {
    A = 5,
    B = 25,
}

enum Second {
    A = 42,
    B = 69,
}

fn main(input: (), witness: ()) {
    let result = First::B + Second::A;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
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
        Instruction::Add(Add),
        Instruction::Store(Store::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::get_instructions(input);

    assert_eq!(expected, result);
}
