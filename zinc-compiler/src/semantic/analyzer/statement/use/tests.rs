//!
//! The `use` statement tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#use::error::Error as UseStatementError;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_expected_path() {
    let input = r#"
use 5;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Statement(
        StatementError::Use(UseStatementError::ExpectedPath {
            location: Location::new(2, 5),
            found: Element::Constant(Constant::Integer(IntegerConstant::new(
                Location::new(2, 5),
                BigInt::from(5),
                false,
                zinc_const::BITLENGTH_BYTE,
            )))
            .to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
