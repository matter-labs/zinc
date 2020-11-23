//!
//! The constant function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
use crate::semantic::scope::item::Item as ScopeItem;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
const fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(2, 1),
        function: "another".to_owned(),
        expected: 1,
        found: 0,
        reference: Some(Location::test(7, 24)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
const fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(1, 2);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(2, 1),
        function: "another".to_owned(),
        expected: 1,
        found: 2,
        reference: Some(Location::test(7, 24)),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_type() {
    let input = r#"
const fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(7, 25),
        function: "another".to_owned(),
        name: "x".to_owned(),
        position: 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_constantness() {
    let input = r#"
const fn another(x: u8) -> u8 {
    42
}

fn main() {
    let x = 42;
    another(x);
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(8, 13),
            found: ScopeItem::Variable(ScopeVariableItem::new(
                Some(Location::test(8, 13)),
                false,
                "x".to_owned(),
                Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            ))
            .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_not_evaluable() {
    let input = r#"
const fn another(x: u8) -> u8 {
    42
}

type X = u8;

fn main() {
    let value = another(X);
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::ExpressionNonConstantElement {
            location: Location::test(9, 25),
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_return_type() {
    let input = r#"
const fn another() -> bool {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionReturnType {
        location: Location::test(3, 5),
        function: "another".to_owned(),
        expected: Type::boolean(None).to_string(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        reference: Location::test(2, 23),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_non_callable_object() {
    let input = r#"
type another = (u8, u8);

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionNonCallable {
        location: Location::test(5, 17),
        name: Element::Type(Type::tuple(
            Some(Location::test(5, 17)),
            vec![Type::integer_unsigned(None, zinc_const::bitlength::BYTE); 2],
        ))
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
