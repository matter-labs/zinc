//!
//! The tuple value element tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_field_out_of_range() {
    let input = r#"
fn main() {
    let result = (true, true, false).5;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TupleFieldOutOfRange {
        location: Location::test(3, 38),
        r#type: Type::tuple(Some(Location::test(3, 38)), vec![Type::boolean(None); 3]).to_string(),
        field_index: 5,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
