//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::element::path::Path;
use crate::semantic::Error as SemanticError;
use crate::syntax::MemberString;
use crate::Error;

#[test]
fn test() {
    let input = r#"
type X = field;

fn main() {
    let data = X {
        value: 42,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::TypeAliasDoesNotPointToStructure(
            Location::new(5, 16),
            Path::new(
                Location::new(5, 16),
                MemberString::new(Location::new(5, 16), "field".to_owned()),
            )
            .to_string(),
        ),
    ));

    let result = super::get_binary_result(input);

    assert_eq!(result, expected);
}
