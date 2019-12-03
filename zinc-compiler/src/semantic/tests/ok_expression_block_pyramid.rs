//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

use crate::semantic::Analyzer;
use crate::syntax::Parser;

#[test]
fn test() {
    let input = r#"
fn main() {
    let pyramid = 1 + {
        2 + {
            3 + {
                4
            } + 3
        } + 2
    } + 1;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(4), false, 8)),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(3), false, 8)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(2), false, 8)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Add(Add),
        Instruction::PushConst(PushConst::new(BigInt::from(1), false, 8)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = Analyzer::default().compile(
        Parser::default()
            .parse(input.to_owned())
            .expect(super::PANIC_SYNTAX_ERROR),
    );

    assert_eq!(expected, result);
}
