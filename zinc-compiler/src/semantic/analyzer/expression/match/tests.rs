//!
//! The match expression tests.
//!

use std::collections::HashMap;
use std::path::PathBuf;

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;
use crate::source::Source;

#[test]
fn ok_boolean() {
    let input = r#"
fn main() -> u8 {
    let condition = true;
    match condition {
        true => 42,
        false => 64,
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_integer() {
    let input = r#"
fn main() -> bool {
    let value = 42;
    match value {
        1 => false,
        2 => false,
        42 => true,
        _ => false,
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_enumeration_two_variants() {
    let input = r#"
enum List {
    A = 1,
    B = 2,
}

fn main() -> u8 {
    let value = List::A;
    match value {
        List::A => 10,
        List::B => 20,
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_enumeration_five_variants() {
    let input = r#"
enum List {
    A = 1,
    B = 2,
    C = 3,
    D = 4,
    E = 5,
}

fn main() -> u8 {
    let value = List::A;
    match value {
        List::A => 10,
        List::B => 20,
        List::C => 30,
        List::D => 40,
        List::E => 50,
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

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

    let expected = Err(Error::Semantic(SemanticError::MatchScrutineeInvalidType {
        location: Location::test(4, 24),
        found: Type::unit(None).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchNotExhausted {
        location: Location::test(4, 18),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchLessThanTwoBranches {
        location: Location::test(4, 18),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchUnreachable {
        location: Location::test(7, 9),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchUnreachable {
        location: Location::test(7, 9),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchUnreachable {
        location: Location::test(14, 9),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_pattern_path_expected_constant() {
    let module_1 = r#"
type X = field;
"#;

    let entry = r#"
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
            location: Location::test(7, 17),
            found: Element::Type(Type::field(None)).to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry_with_modules(
        entry,
        vec![(
            "module_1".to_owned(),
            Source::test(module_1, PathBuf::from("module_1.zn"), HashMap::new())
                .expect(zinc_const::panic::TEST_DATA_VALID),
        )]
        .into_iter()
        .collect::<HashMap<String, Source>>(),
    );

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

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternInvalidType {
            location: Location::test(5, 9),
            expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            found: Type::boolean(None).to_string(),
            reference: Location::test(4, 24),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_branch_pattern_invalid_enum() {
    let input = r#"
enum ListOne {
    A = 1,
    B = 2,
    C = 3,
}

enum ListTwo {
    A = 1,
    B = 2,
    C = 3,
}

fn main() {
    let scrutinee = ListOne::A;
    let result = match scrutinee {
        ListOne::A => 10,
        ListTwo::B => 20,
        ListOne::C => 30,
    };
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchPatternInvalidType {
            location: Location::test(18, 9),
            expected: "enumeration ListOne".to_owned(),
            found: "enumeration ListTwo".to_owned(),
            reference: Location::test(16, 24),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::MatchBranchExpressionInvalidType {
            location: Location::test(6, 14),
            expected: Type::boolean(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            reference: Location::test(5, 14),
        },
    ));

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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchDuplicate {
        location: Location::test(6, 9),
        reference: Location::test(5, 9),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::MatchBranchDuplicate {
        location: Location::test(6, 9),
        reference: Location::test(5, 9),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
