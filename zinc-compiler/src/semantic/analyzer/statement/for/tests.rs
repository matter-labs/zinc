//!
//! The `for` statement tests.
//!

#![cfg(test)]

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#for::error::Error as ForStatementError;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_bounds_expected_constant_range_expression() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in true {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(
        StatementError::For(ForStatementError::BoundsExpectedConstantRangeExpression {
            location: Location::new(4, 14),
            found: Constant::Boolean(BooleanConstant::new(true)).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_while_expected_boolean_condition() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in 0..10 while 42 {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(
        StatementError::For(ForStatementError::WhileExpectedBooleanCondition {
            location: Location::new(4, 26),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
