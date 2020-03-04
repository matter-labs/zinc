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

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
