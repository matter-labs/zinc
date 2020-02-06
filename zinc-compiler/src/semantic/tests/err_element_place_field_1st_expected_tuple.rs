//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::ElementError;
use crate::semantic::Error as SemanticError;
use crate::semantic::PlaceError;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
struct Data {
    a: u8,
}

fn main() {
    let data = Data {
        a: 0,
    };
    let value = data.0;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(10, 21),
        ElementError::Place(PlaceError::OperatorFieldFirstOperandExpectedTuple(
            Type::new_structure(
                "Data".to_owned(),
                vec![(
                    "a".to_owned(),
                    Type::new_integer_unsigned(crate::BITLENGTH_BYTE),
                )],
                None,
            )
            .to_string(),
        )),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
