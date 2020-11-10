//!
//! The `use` statement tests.
//!

use num::BigInt;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_associated_constant() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
}

impl Data {
    const C: u8 = 42;
}

use Data::C;

fn main() -> u8 {
    C
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_associated_variant() {
    let input = r#"
enum Data {
    A = 1,
    B = 2,
}

use Data::B;

fn main() -> u8 {
    (Data::A + B) as u8
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_associated_method() {
    let input = r#"
struct Data {
    a: i8,
    b: i8,
}

impl Data {
    pub fn method() -> u8 {
        42
    }
}

use Data::method;

fn main() -> u8 {
    method()
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_expected_path() {
    let input = r#"
use 5;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::UseStatementExpectedPath {
        location: Location::test(2, 5),
        found: Element::Constant(Constant::Integer(IntegerConstant::new(
            Location::test(2, 5),
            BigInt::from(5),
            false,
            zinc_const::bitlength::BYTE,
            true,
        )))
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
