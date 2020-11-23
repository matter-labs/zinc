//!
//! The `<Contract>::transfer` intrinsic function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::function::intrinsic::contract_transfer::Function as ContractTransferFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_contract_transfer_argument_count_lesser() {
    let input = r#"
contract Test {
    pub fn test(mut self) {
        self.transfer(0x42 as u160, 0x0 as u160);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(4, 22),
        function: ContractTransferFunction::IDENTIFIER.to_owned(),
        expected: ContractTransferFunction::ARGUMENT_COUNT,
        found: ContractTransferFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_transfer_argument_count_greater() {
    let input = r#"
contract Test {
    pub fn test(mut self) {
        self.transfer(0x42 as u160, 0x0 as u160, 500 as u248, 0x666);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(4, 22),
        function: ContractTransferFunction::IDENTIFIER.to_owned(),
        expected: ContractTransferFunction::ARGUMENT_COUNT,
        found: ContractTransferFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_transfer_argument_2_recipient_expected_u160() {
    let input = r#"
contract Test {
    pub fn test(mut self) {
        self.transfer(false, 0x0 as u160, 500 as u248);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(4, 23),
        function: ContractTransferFunction::IDENTIFIER.to_owned(),
        name: "recipient".to_owned(),
        position: ContractTransferFunction::ARGUMENT_INDEX_RECIPIENT + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_transfer_argument_3_token_address_expected_u160() {
    let input = r#"
contract Test {
    pub fn test(mut self) {
        self.transfer(0x42 as u160, false, 500 as u248);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(4, 37),
        function: ContractTransferFunction::IDENTIFIER.to_owned(),
        name: "token_address".to_owned(),
        position: ContractTransferFunction::ARGUMENT_INDEX_TOKEN_ADDRESS + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_transfer_argument_4_amount_expected_u248() {
    let input = r#"
contract Test {
    pub fn test(mut self) {
        self.transfer(0x42 as u160, 0x0, false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(4, 42),
        function: ContractTransferFunction::IDENTIFIER.to_owned(),
        name: "amount".to_owned(),
        position: ContractTransferFunction::ARGUMENT_INDEX_AMOUNT + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::INTEGER_MAX).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
