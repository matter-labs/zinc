//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

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
            Type::structure(
                "Data".to_owned(),
                1,
                vec![(
                    "a".to_owned(),
                    Type::integer_unsigned(crate::BITLENGTH_BYTE),
                )],
                None,
            )
            .to_string(),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
