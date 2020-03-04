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
        SemanticError::TypeAliasDoesNotPointToStructure {
            location: Location::new(5, 16),
            found: Path::new(
                Location::new(5, 16),
                MemberString::new(Location::new(5, 16), "field".to_owned()),
            )
            .to_string(),
        },
    ));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
