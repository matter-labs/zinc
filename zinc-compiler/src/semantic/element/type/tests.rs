//!
//! The type tests.
//!

use zinc_lexical::Location;
use zinc_syntax::Identifier;

use crate::error::Error;
use crate::semantic::element::path::Path;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Element;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_type_required() {
    let input = r#"
fn main(a: u8, b: field, mut c) -> u8 {
    42
}
"#;

    let expected = Err(Error::Semantic(SemanticError::BindingTypeRequired {
        location: Location::test(2, 30),
        identifier: "c".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_alias_does_not_point_to_type() {
    let input = r#"
fn main() {
    let unknown = 0;
    let result = 42 as unknown;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeAliasExpectedType {
        location: Location::test(4, 24),
        found: Element::Path(Path::new(
            Location::test(4, 24),
            Identifier::new(Location::test(4, 24), "unknown".to_owned()),
        ))
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_instantiation_forbidden_let() {
    let input = r#"
use std::collections::MTreeMap;

fn main() {
    let map = MTreeMap;
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInstantiationForbidden {
        location: Location::test(5, 9),
        found: "structure MTreeMap".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_instantiation_forbidden_wrapped() {
    let input = r#"
use std::collections::MTreeMap;

struct MapWrapper {
    map: MTreeMap<u8, field>,
}

contract Test {
    wrapper: MapWrapper;

    pub fn default() -> u8 {
        42
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInstantiationForbidden {
        location: Location::test(4, 1),
        found: "structure MapWrapper".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_instantiation_forbidden_function_argument() {
    let input = r#"
use std::collections::MTreeMap;

fn main(map: MTreeMap<u8, field>) {}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInstantiationForbidden {
        location: Location::test(4, 9),
        found: "structure MTreeMap".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_instantiation_forbidden_function_result_type() {
    let input = r#"
use std::collections::MTreeMap;

fn main() -> MTreeMap<u8, field> { MTreeMap }
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInstantiationForbidden {
        location: Location::test(4, 14),
        found: "structure MTreeMap".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_instantiation_forbidden_generic() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    map: MTreeMap<u8, MTreeMap<u8, u8>>;

    pub fn default() -> u8 {
        42
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeInstantiationForbidden {
        location: Location::test(5, 5),
        found: "structure MTreeMap".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_unexpected_generics() {
    let input = r#"
type Array = [u8; 42];

type Invalid = Array<bool>;

fn main() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::TypeUnexpectedGenerics {
        location: Location::test(4, 16),
        r#type: Type::array(
            None,
            Type::integer_unsigned(None, zinc_const::bitlength::BYTE),
            42,
        )
        .to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
