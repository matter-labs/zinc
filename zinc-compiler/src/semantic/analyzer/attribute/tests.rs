//!
//! The attribute tests.
//!

use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::error::Error as SemanticError;

#[test]
fn ok_test() {
    let input = r#"
fn main() {}

#[test]
fn test() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_should_panic() {
    let input = r#"
fn main() {}

#[should_panic]
fn test() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_ignore() {
    let input = r#"
fn main() {}

#[ignore]
fn test() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn ok_multiple() {
    let input = r#"
fn main() {}

#[test]
#[should_panic]
#[ignore]
fn test() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_unknown() {
    let input = r#"
fn main() {}

#[unknown]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeUnknown {
        location: Location::test(4, 1),
        found: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_empty() {
    let input = r#"
fn main() {}

#[]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeEmpty {
        location: Location::test(4, 1),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn ok_zksync_msg() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = 1.0_E18,
)]
fn test() {}
"#;

    assert!(crate::semantic::tests::compile_entry(input).is_ok());
}

#[test]
fn error_elements_count_zksync_msg() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeElementsCount {
        location: Location::test(4, 3),
        name: "zksync::msg".to_owned(),
        expected: zinc_const::contract::TRANSACTION_FIELDS_COUNT,
        found: zinc_const::contract::TRANSACTION_FIELDS_COUNT - 1,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_nested_zksync_msg() {
    let input = r#"
fn main() {}

#[zksync::msg]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeExpectedNested {
        location: Location::test(4, 3),
        name: "zksync::msg".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_element_zksync_msg_sender() {
    let input = r#"
fn main() {}

#[zksync::msg(
    unknown = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeExpectedElement {
        location: Location::test(5, 5),
        name: "zksync::msg".to_owned(),
        position: 1,
        expected: "sender".to_owned(),
        found: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_integer_literal_zksync_msg_sender() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = false,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::AttributeExpectedIntegerLiteral {
            location: Location::test(5, 5),
            name: "sender".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_zksync_msg_sender() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x10000000000000000000000000000000000000000,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::InvalidInteger {
        location: Location::test(5, 14),
        inner: zinc_math::Error::Overflow {
            value: zinc_math::bigint_from_str("0x10000000000000000000000000000000000000000")
                .expect(zinc_const::panic::TEST_DATA_VALID),
            is_signed: false,
            bitlength: zinc_const::bitlength::ETH_ADDRESS,
        },
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_element_zksync_msg_recipient() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    unknown = 0x0002,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeExpectedElement {
        location: Location::test(6, 5),
        name: "zksync::msg".to_owned(),
        position: 2,
        expected: "recipient".to_owned(),
        found: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_integer_literal_zksync_msg_recipient() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = false,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::AttributeExpectedIntegerLiteral {
            location: Location::test(6, 5),
            name: "recipient".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_zksync_msg_recipient() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x10000000000000000000000000000000000000000,
    token_address = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::InvalidInteger {
        location: Location::test(6, 17),
        inner: zinc_math::Error::Overflow {
            value: zinc_math::bigint_from_str("0x10000000000000000000000000000000000000000")
                .expect(zinc_const::panic::TEST_DATA_VALID),
            is_signed: false,
            bitlength: zinc_const::bitlength::ETH_ADDRESS,
        },
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_element_zksync_msg_token_address() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    unknown = 0x0003,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeExpectedElement {
        location: Location::test(7, 5),
        name: "zksync::msg".to_owned(),
        position: 3,
        expected: "token_address".to_owned(),
        found: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_integer_literal_zksync_msg_token_address() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = false,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::AttributeExpectedIntegerLiteral {
            location: Location::test(7, 5),
            name: "token_address".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_zksync_msg_token_address() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x10000000000000000000000000000000000000000,
    amount = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::InvalidInteger {
        location: Location::test(7, 21),
        inner: zinc_math::Error::Overflow {
            value: zinc_math::bigint_from_str("0x10000000000000000000000000000000000000000")
                .expect(zinc_const::panic::TEST_DATA_VALID),
            is_signed: false,
            bitlength: zinc_const::bitlength::ETH_ADDRESS,
        },
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_element_zksync_msg_amount() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
    unknown = 1000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::AttributeExpectedElement {
        location: Location::test(8, 5),
        name: "zksync::msg".to_owned(),
        position: 4,
        expected: "amount".to_owned(),
        found: "unknown".to_owned(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_expected_integer_literal_zksync_msg_amount() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = false,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::AttributeExpectedIntegerLiteral {
            location: Location::test(8, 5),
            name: "amount".to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_overflow_zksync_msg_amount() {
    let input = r#"
fn main() {}

#[zksync::msg(
    sender = 0x0001,
    recipient = 0x0002,
    token_address = 0x0003,
    amount = 0x100000000000000000000000000000000000000000000000000000000000000,
)]
fn test() {}
"#;

    let expected = Err(Error::Semantic(SemanticError::InvalidInteger {
        location: Location::test(8, 14),
        inner: zinc_math::Error::Overflow {
            value: zinc_math::bigint_from_str(
                "0x100000000000000000000000000000000000000000000000000000000000000",
            )
            .expect(zinc_const::panic::TEST_DATA_VALID),
            is_signed: false,
            bitlength: zinc_const::bitlength::BALANCE,
        },
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
