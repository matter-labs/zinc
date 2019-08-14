//!
//! The syntax tests.
//!

#![cfg(test)]

use crate::lexical::Identifier;
use crate::lexical::TokenStream;

use super::*;

#[test]
fn success() {
    let code = br#"/*
    This is the mega ultra inputs input!
*/
inputs {
    a: uint8; // input 1
    b: field; // input 2
    c: bool; // input 3
} /* This is the end of the mega ultra inputs input */

/*
    This is the mega ultra witness input!
*/
witness {
    d: int126; // witness 1
    e: field; // witness 2
    f: bool; // witness 3
} /* This is the end of the mega ultra witness input */"#;

    let result: CircuitProgram = parse(TokenStream::new(code.to_vec())).unwrap();

    let expected: CircuitProgram = CircuitProgram {
        inputs: vec![
            Input::new(Identifier("a".to_string()), Type::Uint { bitlength: 8 }),
            Input::new(Identifier("b".to_string()), Type::Field),
            Input::new(Identifier("c".to_string()), Type::Bool),
        ],
        witnesses: vec![
            Witness::new(Identifier("d".to_string()), Type::Int { bitlength: 126 }),
            Witness::new(Identifier("e".to_string()), Type::Field),
            Witness::new(Identifier("f".to_string()), Type::Bool),
        ],
        statements: vec![],
    };

    assert_eq!(result, expected);
}
