//!
//! The statement tests.
//!

#![cfg(test)]

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::Error as SemanticError;

#[test]
fn error_loop_bounds_expected_constant_range_expression() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in true {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::LoopBoundsExpectedConstantRangeExpression {
            location: Location::new(4, 14),
            found: Constant::Boolean(true).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_loop_while_expected_boolean_condition() {
    let input = r#"
fn main() {
    let mut sum = 0;
    for i in 0..10 while 42 {
        sum = sum + i;
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::LoopWhileExpectedBooleanCondition {
            location: Location::new(4, 26),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_structure_duplicate_field() {
    let input = r#"
struct Data {
    a: u8,
    b: u8,
    b: field,
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::StructureDuplicateField {
        location: Location::new(5, 5),
        type_identifier: "Data".to_owned(),
        field_name: "b".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_impl_expected_structure_or_enumeration() {
    let input = r#"
type X = field;

impl X {
    fn impossible() {}
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ImplStatementExpectedStructureOrEnumeration {
            location: Location::new(4, 6),
            found: Type::field().to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_module_not_found() {
    let input = r#"
mod unknown;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::ModuleNotFound {
        location: Location::new(2, 5),
        name: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_use_expected_path() {
    let input = r#"
use 5;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::UseExpectedPath {
        location: Location::new(2, 5),
        found: IntegerConstant::new(BigInt::from(5), false, crate::BITLENGTH_BYTE).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
