//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;
use crate::semantic::Error as SemanticError;
use crate::semantic::Path;
use crate::syntax::MemberString;
use crate::Error;

#[test]
fn test() {
    let input = r#"
fn main(input: (), witness: ()) {
    let unknown = 0;
    let result = 42 as unknown;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeAliasDoesNotPointToType(
        Location::new(4, 24),
        Path::new(
            Location::new(4, 24),
            MemberString::new(Location::new(4, 24), "unknown".to_owned()),
        )
        .to_string(),
    )));

    let result = super::get_binary_result(input);

    assert_eq!(expected, result);
}
