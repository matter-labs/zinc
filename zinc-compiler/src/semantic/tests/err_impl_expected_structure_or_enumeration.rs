//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = field;

impl X {
    fn impossible() {}
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ImplStatementExpectedStructureOrEnumeration(
            Location::new(4, 6),
            "field".to_owned(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
