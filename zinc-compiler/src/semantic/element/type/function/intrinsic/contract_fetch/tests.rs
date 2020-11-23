//!
//! The `<Contract>::fetch` intrinsic function tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::r#type::function::intrinsic::contract_fetch::Function as ContractFetchFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_contract_fetch_argument_count_lesser() {
    let input = r#"
contract Test {
    pub fn external(self, address: u160) {
        let inner = Self::fetch();
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(4, 21),
        function: ContractFetchFunction::IDENTIFIER.to_owned(),
        expected: ContractFetchFunction::ARGUMENT_COUNT,
        found: ContractFetchFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_fetch_argument_count_greater() {
    let input = r#"
contract Test {
    pub fn external(self, address: u160) {
        let inner = Self::fetch(address, false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(4, 21),
        function: ContractFetchFunction::IDENTIFIER.to_owned(),
        expected: ContractFetchFunction::ARGUMENT_COUNT,
        found: ContractFetchFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_contract_fetch_argument_1_address_expected_u160() {
    let input = r#"
contract Test {
    pub fn external(self, address: u160) {
        let inner = Self::fetch(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(4, 33),
        function: ContractFetchFunction::IDENTIFIER.to_owned(),
        name: "address".to_owned(),
        position: ContractFetchFunction::ARGUMENT_INDEX_ADDRESS + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
