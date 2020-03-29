//!
//! The conditional expression tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn error_conditional_branch_types_mismatch() {
    let input = r#"
fn main() {
    if true { 42 } else { false }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConditionalBranchTypesMismatch {
            location: Location::new(3, 15),
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
            reference: Location::new(3, 27),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_conditional_expected_boolean_condition() {
    let input = r#"
fn main() {
    if 42 { 1 } else { 2 }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ConditionalExpectedBooleanCondition {
            location: Location::new(3, 8),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
