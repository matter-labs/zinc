//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::Error as SemanticError;

use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main() {
    let value = [true, true, false].first;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 36),
        ElementError::Value(ValueError::OperatorFieldFirstOperandExpectedStructure(
            Type::array(Type::boolean(), 3).to_string(),
        )),
    )));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
