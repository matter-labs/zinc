//!
//! The runtime function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_argument_count_lesser() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(2, 1),
            function: "another".to_owned(),
            expected: 1,
            found: 0,
            reference: Some(Location::test(7, 24)),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_count_greater() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(1, 2);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(2, 1),
            function: "another".to_owned(),
            expected: 1,
            found: 2,
            reference: Some(Location::test(7, 24)),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_type() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

fn main() {
    let value = another(false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(7, 25),
            function: "another".to_owned(),
            name: "x".to_owned(),
            position: 1,
            expected: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            found: Type::boolean(None).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_constantness() {
    let input = r#"
fn main() -> [u8; 2] {
    let array = [1, 2, 3, 4];
    let new_length = 2;
    std::array::truncate(array, new_length)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentConstantness {
            location: Location::test(5, 33),
            function: "truncate".to_owned(),
            name: "new_length".to_owned(),
            position: 2,
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_argument_not_evaluable() {
    let input = r#"
fn another(x: u8) -> u8 {
    42
}

type X = u8;

fn main() {
    let value = another(X);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentNotEvaluable {
            location: Location::test(9, 25),
            function: "another".to_owned(),
            position: 1,
            found: Element::Type(Type::integer_unsigned(None, zinc_const::bitlength::BYTE))
                .to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_return_type() {
    let input = r#"
fn another() -> bool {
    42
}

fn main() {
    let value = another();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ReturnType {
            location: Location::test(3, 5),
            function: "another".to_owned(),
            expected: Type::boolean(None).to_string(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
            reference: Location::test(2, 17),
        }),
    ))));

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

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::NonCallable {
            location: Location::test(5, 17),
            name: Element::Type(Type::tuple(
                Some(Location::test(5, 17)),
                vec![Type::integer_unsigned(None, zinc_const::bitlength::BYTE); 2],
            ))
            .to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_function_method_self_not_first() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    fn method(value: u8, self) -> u8 {
        value
    }
}

fn main() {
    let data = Data { value: 42 };
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::FunctionMethodSelfNotFirst {
            location: Location::test(7, 8),
            function: "method".to_owned(),
            position: 2,
            reference: Location::test(7, 26),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn ok_calling_mutable_from_immutable_structure() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    pub fn not_immutable(mut self) -> u8 {
        self.mutable()
    }

    pub fn mutable(mut self) -> u8 {
        self.value = 0;
        self.value
    }
}

fn main() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_calling_mutable_from_immutable_structure() {
    let input = r#"
struct Data {
    value: u8,
}

impl Data {
    pub fn immutable(self) -> u8 {
        self.mutable()
    }

    pub fn mutable(mut self) -> u8 {
        self.value = 0;
        self.value
    }
}

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::CallingMutableFromImmutable {
            location: Location::test(8, 21),
            function: "mutable".to_owned(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn ok_calling_mutable_from_immutable_contract() {
    let input = r#"
contract Data {
    value: u8;

    pub fn not_immutable(mut self) -> u8 {
        self.mutable()
    }

    pub fn mutable(mut self) -> u8 {
        self.value = 0;
        self.value
    }
}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_calling_mutable_from_immutable_contract() {
    let input = r#"
contract Data {
    value: u8;

    pub fn immutable(self) -> u8 {
        self.mutable()
    }

    pub fn mutable(mut self) -> u8 {
        self.value = 0;
        self.value
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::CallingMutableFromImmutable {
            location: Location::test(6, 21),
            function: "mutable".to_owned(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
