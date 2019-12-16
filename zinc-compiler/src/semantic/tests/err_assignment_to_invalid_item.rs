//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use crate::lexical::Location;

use crate::semantic::Error as SemanticError;
use crate::semantic::ScopeItem;
use crate::semantic::Type;

use crate::Error;

#[test]
fn test() {
    let input = r#"
type INVALID = field;

fn main() {
    INVALID = 25;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::AssignmentToInvalidItem(
        Location::new(5, 5),
        ScopeItem::Type(Type::Field).to_string(),
    )));

    let result = super::result(input);

    assert_eq!(expected, result);
}
