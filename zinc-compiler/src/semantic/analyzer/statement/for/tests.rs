//!
//! The `for` statement tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::constant::boolean::Boolean as BooleanConstant;
use crate::semantic::element::constant::Constant;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_ordinar() {
    let input = r#"
fn main() {
    for i in 0..10 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_ordinar_with_while() {
    let input = r#"
fn main() {
    for i in 0..10 while i < 5 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_inclusive() {
    let input = r#"
fn main() {
    for i in 0..=10 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_inclusive_with_while() {
    let input = r#"
fn main() {
    for i in 0..=10 while i < 5 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_reversed() {
    let input = r#"
fn main() {
    for i in 10..0 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_reversed_with_while() {
    let input = r#"
fn main() {
    for i in 10..0 while i > 5 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_reversed_inclusive() {
    let input = r#"
fn main() {
    for i in 10..=0 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_reversed_inclusive_with_while() {
    let input = r#"
fn main() {
    for i in 10..=0 while i > 5 {
        dbg!("{}", i);
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

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

    let expected = Err(Error::Semantic(
        SemanticError::ForStatementBoundsExpectedConstantRangeExpression {
            location: Location::test(4, 14),
            found: Element::Constant(Constant::Boolean(BooleanConstant::new(
                Location::test(4, 14),
                true,
            )))
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::ForStatementWhileExpectedBooleanCondition {
            location: Location::test(4, 26),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
