//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::Error as SemanticError;

#[test]
fn test() {
    let input = r#"
use 5;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::UseExpectedPath {
        location: Location::new(2, 5),
        found: IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
    }));

    let result = super::compile_entry_point(input);

    assert_eq!(result, expected);
}
