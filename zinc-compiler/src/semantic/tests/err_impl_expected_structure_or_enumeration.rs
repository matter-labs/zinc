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
        SemanticError::ImplStatementExpectedStructureOrEnumeration {
            location: Location::new(4, 6),
            found: "field".to_owned(),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
