//!
//! The test function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_call_forbidden() {
    let input = r#"
#[test]
fn test() {
    require(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::UnitTestCallForbidden {
        location: Location::test(8, 17),
        function: "test".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_beyond_module_scope() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    #[test]
    fn test() {
        require(true);
    }
}

fn main() -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::UnitTestBeyondModuleScope {
        location: Location::test(8, 5),
        function: "test".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_public_forbidden() {
    let input = r#"
#[test]
pub fn test() {
    require(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::UnitTestPublicForbidden {
        location: Location::test(3, 1),
        function: "test".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_constant_forbidden() {
    let input = r#"
#[test]
const fn test() {
    require(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::UnitTestConstantForbidden {
        location: Location::test(3, 1),
        function: "test".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_cannot_have_arguments() {
    let input = r#"
#[test]
fn test(value: u8) {
    require(true);
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::UnitTestCannotHaveArguments {
            location: Location::test(3, 1),
            function: "test".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_cannot_return_value() {
    let input = r#"
#[test]
fn test() -> u8 {
    require(true);
    42
}

fn main() {
    let value = test();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::UnitTestCannotReturnValue {
        location: Location::test(3, 1),
        function: "test".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
