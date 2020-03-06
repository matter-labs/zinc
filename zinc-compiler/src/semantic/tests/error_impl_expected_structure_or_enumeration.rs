//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

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
            found: Type::field().to_string(),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
