//!
//! The intrinsic function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_exclamation_mark_missing() {
    let input = r#"
fn main() {
    dbg();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(
            IntrinsicFunctionError::ExclamationMarkMissing {
                location: Location::test(3, 5),
                function: "dbg",
            },
        )),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_unknown() {
    let input = r#"
fn unknown() {}

fn main() {
    unknown!();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Unknown {
            location: Location::test(5, 13),
            function: "unknown".to_owned(),
        })),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
