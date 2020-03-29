//!
//! The type tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;
use crate::syntax::MemberString;

#[test]
fn error_type_alias_does_not_point_to_structure() {
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
                MemberString::new(Location::new(5, 16), Type::field().to_string()),
            )
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_type_alias_does_not_point_to_type() {
    let input = r#"
fn main() {
    let unknown = 0;
    let result = 42 as unknown;
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::TypeAliasDoesNotPointToType {
            location: Location::new(4, 24),
            found: Path::new(
                Location::new(4, 24),
                MemberString::new(Location::new(4, 24), "unknown".to_owned()),
            )
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
