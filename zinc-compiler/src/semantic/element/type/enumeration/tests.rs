//!
//! The semantic analyzer enumeration type element tests.
//!

use num::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::r#type::enumeration::error::Error as EnumerationTypeError;
use crate::semantic::element::r#type::error::Error as TypeError;
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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Enumeration(EnumerationTypeError::DuplicateVariantValue {
            location: Location::test(4, 5),
            type_identifier: "List".to_owned(),
            variant_name: "B".to_owned(),
            variant_value: BigInt::from(2),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
