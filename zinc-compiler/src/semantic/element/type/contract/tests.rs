//!
//! The semantic analyzer contract type element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_duplicate_field() {
    let input = r#"
contract Contract {
    a: u8;
    b: u8;
    b: field;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::ScopeItemRedeclared {
        location: Location::test(5, 5),
        name: "b".to_owned(),
        reference: Some(Location::test(4, 5)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
