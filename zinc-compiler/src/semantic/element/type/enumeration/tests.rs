//!
//! The semantic analyzer enumeration type element tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_duplicate_field() {
    let input = r#"
enum List {
    A = 1,
    B = 2,
    C = 2,
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeDuplicateVariantValue {
        location: Location::test(4, 5),
        r#type: "List".to_owned(),
        variant_name: "B".to_owned(),
        variant_value: BigInt::from(2),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
