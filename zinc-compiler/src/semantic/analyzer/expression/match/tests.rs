//!
//! The match expression tests.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::expression::r#match::error::Error as MatchExpressionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::Scope;

#[test]
fn error_scrutinee_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = ();
    let result = match () {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::ScrutineeInvalidType {
            location: Location::new(4, 24),
            found: Type::unit().to_string(),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_not_exhausted() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        1 => 10,
        2 => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::NotExhausted {
            location: Location::new(4, 18),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_less_than_two_branches() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        _ => 10,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::LessThanTwoBranches {
            location: Location::new(4, 18),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_unreachable() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        1 => 10,
        _ => 101,
        2 => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchUnreachable {
            location: Location::new(7, 9),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_unreachable_exhausted_boolean() {
    let input = r#"
fn main() {
    let scrutinee = true;
    let result = match scrutinee {
        false => 10,
        true => 101,
        _ => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchUnreachable {
            location: Location::new(7, 9),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_unreachable_exhausted_enumeration() {
    let input = r#"
enum List {
    One = 1,
    Two = 2,
    Three = 3,
}

fn main() {
    let scrutinee = List::One;
    let result = match scrutinee {
        List::One => 1,
        List::Two => 2,
        List::Three => 3,
        _ => 4,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchUnreachable {
            location: Location::new(14, 9),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_pattern_path_expected_constant() {
    let module_1 = r#"
type X = field;
"#;

    let binary = r#"
mod module_1;

fn main() -> u8 {
    let value = 42;
    match value {
        module_1::X => 1,
        _ => 0,
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchPatternPathExpectedConstant {
            location: Location::new(7, 17),
            found: Type::field().to_string(),
        }),
    )));

    let module_1 = crate::semantic::tests::compile_module(module_1).expect(crate::panic::TEST_DATA);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![("module_1".to_owned(), module_1)]
        .into_iter()
        .collect();

    let result = crate::semantic::tests::compile_entry_with_dependencies(binary, dependencies);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_pattern_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        false => 0,
        true => 1,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchPatternInvalidType {
            location: Location::new(5, 9),
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
            reference: Location::new(4, 24),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_expression_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchExpressionInvalidType {
            location: Location::new(6, 14),
            expected: Type::boolean().to_string(),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            reference: Location::new(5, 14),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_duplicate_boolean() {
    let input = r#"
fn main() {
    let scrutinee = true;
    let result = match scrutinee {
        false => 10,
        false => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchDuplicate {
            location: Location::new(6, 9),
            reference: Location::new(5, 9),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_duplicate_integer() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        42 => 10,
        42 => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Expression(
        ExpressionError::Match(MatchExpressionError::BranchDuplicate {
            location: Location::new(6, 9),
            reference: Location::new(5, 9),
        }),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
