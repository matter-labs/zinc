//!
//! The intrinsic function tests.
//!

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::zksync::transfer::Function as ZksyncTransferFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_transfer_argument_count_lesser() {
    let input = r#"
fn main() {
    zksync::transfer(0x42 as u160, 1);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: ZksyncTransferFunction::IDENTIFIER.to_owned(),
            expected: ZksyncTransferFunction::ARGUMENT_COUNT,
            found: ZksyncTransferFunction::ARGUMENT_COUNT - 1,
            reference: None,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_transfer_argument_count_greater() {
    let input = r#"
fn main() {
    zksync::transfer(0x42 as u160, 1, 500 as u248, 0x666);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentCount {
            location: Location::test(3, 5),
            function: ZksyncTransferFunction::IDENTIFIER.to_owned(),
            expected: ZksyncTransferFunction::ARGUMENT_COUNT,
            found: ZksyncTransferFunction::ARGUMENT_COUNT + 1,
            reference: None,
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_transfer_argument_1_recipient_expected_u160() {
    let input = r#"
fn main() {
    zksync::transfer(false, 1, 500 as u248);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 22),
            function: ZksyncTransferFunction::IDENTIFIER.to_owned(),
            name: "recipient".to_owned(),
            position: ZksyncTransferFunction::ARGUMENT_INDEX_RECIPIENT + 1,
            expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
            found: Type::boolean(None).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_transfer_argument_2_token_id_expected_unsigned_integer() {
    let input = r#"
fn main() {
    zksync::transfer(0x42 as u160, false, 500 as u248);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 36),
            function: ZksyncTransferFunction::IDENTIFIER.to_owned(),
            name: "token_id".to_owned(),
            position: ZksyncTransferFunction::ARGUMENT_INDEX_TOKEN_ID + 1,
            expected: "{unsigned integer}".to_owned(),
            found: Type::boolean(None).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_transfer_argument_3_amount_expected_u248() {
    let input = r#"
fn main() {
    zksync::transfer(0x42 as u160, 1, false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(ElementError::Type(
        TypeError::Function(FunctionError::ArgumentType {
            location: Location::test(3, 39),
            function: ZksyncTransferFunction::IDENTIFIER.to_owned(),
            name: "amount".to_owned(),
            position: ZksyncTransferFunction::ARGUMENT_INDEX_AMOUNT + 1,
            expected: Type::integer_unsigned(None, zinc_const::bitlength::INTEGER_MAX).to_string(),
            found: Type::boolean(None).to_string(),
        }),
    ))));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
