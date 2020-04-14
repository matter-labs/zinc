//!
//! The conditional expression tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::expression::conditional::error::Error as ConditionalExpressionError;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_branch_types_mismatch() {
    let input = r#"
fn main() {
    if true { 42 } else { false }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Conditional(ConditionalExpressionError::BranchTypesMismatch {
            location: Location::new(3, 15),
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
            reference: Location::new(3, 27),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_boolean_condition() {
    let input = r#"
fn main() {
    if 42 { 1 } else { 2 }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Conditional(ConditionalExpressionError::ExpectedBooleanCondition {
            location: Location::new(3, 8),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
