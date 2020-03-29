//!
//! The match expression tests.
//!

#![cfg(test)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Location;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::Scope;
use crate::semantic::Error as SemanticError;

static PANIC_COMPILE_DEPENDENCY: &str = "Dependencies must be successfully compiled";

#[test]
fn error_match_scrutinee_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = ();
    let result = match () {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MatchScrutineeInvalidType {
        location: Location::new(4, 24),
        found: Type::unit().to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_match_branch_expression_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        0 => false,
        1 => 0,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchExpressionInvalidType {
            location: Location::new(6, 14),
            expected: Type::boolean().to_string(),
            found: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            reference: Location::new(5, 14),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_match_branch_pattern_invalid_type() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        false => 0,
        true => 1,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternInvalidType {
            location: Location::new(5, 9),
            expected: Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            found: Type::boolean().to_string(),
            reference: Location::new(4, 24),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_match_branch_pattern_path_expected_constant() {
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

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternPathExpectedConstant {
            location: Location::new(7, 17),
            found: Type::field().to_string(),
        },
    ));

    let module_1 =
        crate::semantic::tests::compile_module(module_1).expect(PANIC_COMPILE_DEPENDENCY);

    let dependencies: HashMap<String, Rc<RefCell<Scope>>> = vec![("module_1".to_owned(), module_1)]
        .into_iter()
        .collect();

    let result = crate::semantic::tests::compile_entry_with_dependencies(binary, dependencies);

    assert_eq!(result, expected);
}

#[test]
fn error_match_branch_unreachable() {
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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchUnreachable {
        location: Location::new(7, 9),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_match_less_than_two_branches() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        _ => 10,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MatchLessThanTwoBranches {
        location: Location::new(4, 18),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_match_not_exhausted() {
    let input = r#"
fn main() {
    let scrutinee = 42;
    let result = match scrutinee {
        1 => 10,
        2 => 20,
    };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::MatchNotExhausted {
        location: Location::new(4, 18),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
