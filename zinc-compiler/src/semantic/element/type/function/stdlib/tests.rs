//!
//! The built-in function tests.
//!

#![cfg(test)]

use std::str::FromStr;

use num_bigint::BigInt;

use crate::error::Error;
use crate::lexical::token::location::Location;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionTypeError;
use crate::semantic::element::r#type::function::stdlib::array_pad::Function as ArrayPadFunction;
use crate::semantic::element::r#type::function::stdlib::array_reverse::Function as ArrayReverseFunction;
use crate::semantic::element::r#type::function::stdlib::array_truncate::Function as ArrayTruncateFunction;
use crate::semantic::element::r#type::function::stdlib::convert_from_bits_field::Function as ConvertFromBitsFieldFunction;
use crate::semantic::element::r#type::function::stdlib::convert_from_bits_signed::Function as ConvertFromBitsSignedFunction;
use crate::semantic::element::r#type::function::stdlib::convert_from_bits_unsigned::Function as ConvertFromBitsUnsignedFunction;
use crate::semantic::element::r#type::function::stdlib::convert_to_bits::Function as ConvertToBitsFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_blake2s::Function as CryptoBlake2sFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_blake2s_multi_input::Function as CryptoBlake2sMultiInputFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_pedersen::Function as CryptoPedersenFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_pedersen_multi_input::Function as CryptoPedersenMultiInputFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_schnorr_signature_verify::Function as CryptoSchnorrSignatureVerifyFunction;
use crate::semantic::element::r#type::function::stdlib::crypto_sha256::Function as CryptoSha256Function;
use crate::semantic::element::r#type::function::stdlib::error::Error as StandardLibraryFunctionTypeError;
use crate::semantic::element::r#type::function::stdlib::ff_invert::Function as FfInvertFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::element::Error as ElementError;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_crypto_sha256_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::sha256();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "sha256".to_owned(),
            CryptoSha256Function::ARGUMENT_COUNT,
            CryptoSha256Function::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_sha256_argument_count_greater() {
    let input = r#"
fn main() {
    std::crypto::sha256([true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "sha256".to_owned(),
            CryptoSha256Function::ARGUMENT_COUNT,
            CryptoSha256Function::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_sha256_argument_1_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::sha256(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "sha256".to_owned(),
            "preimage".to_owned(),
            CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_sha256_argument_1_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::sha256([true; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "sha256".to_owned(),
            "preimage".to_owned(),
            CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_sha256_argument_1_preimage_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::crypto::sha256([true; 4]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "sha256".to_owned(),
            "preimage".to_owned(),
            CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::pedersen();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pedersen".to_owned(),
            CryptoPedersenFunction::ARGUMENT_COUNT,
            CryptoPedersenFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_argument_count_greater() {
    let input = r#"
fn main() {
    std::crypto::pedersen([true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pedersen".to_owned(),
            CryptoPedersenFunction::ARGUMENT_COUNT,
            CryptoPedersenFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_argument_1_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::pedersen(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen".to_owned(),
            "preimage".to_owned(),
            CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_argument_1_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::pedersen([true; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen".to_owned(),
            "preimage".to_owned(),
            CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_argument_1_preimage_expected_bit_array_size_limit() {
    let input = r#"
fn main() {
    std::crypto::pedersen([true; 513]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen".to_owned(),
            "preimage".to_owned(),
            CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), crate::LIMIT_PEDERSEN_HASH_INPUT_BITS + 1).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pedersen_multi_input".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_COUNT,
            CryptoPedersenMultiInputFunction::ARGUMENT_COUNT - 2,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_count_greater() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true; 8], [true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pedersen_multi_input".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_COUNT,
            CryptoPedersenMultiInputFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_1_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input(42, [true;8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_2_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true;8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_1_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true; 0], [true; 8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_2_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true; 8], [true; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_1_preimage_expected_bit_array_size_limit() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true; 513], [true; 8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), crate::LIMIT_PEDERSEN_HASH_INPUT_BITS + 1).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_pedersen_multi_input_argument_2_preimage_expected_bit_array_size_limit() {
    let input = r#"
fn main() {
    std::crypto::pedersen_multi_input([true; 8], [true; 513]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 38),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pedersen_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoPedersenMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!(
                "[bool; N], 0 < N <= {}",
                crate::LIMIT_PEDERSEN_HASH_INPUT_BITS
            ),
            Type::array(Type::boolean(), crate::LIMIT_PEDERSEN_HASH_INPUT_BITS + 1).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::blake2s();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "blake2s".to_owned(),
            CryptoBlake2sFunction::ARGUMENT_COUNT,
            CryptoBlake2sFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_argument_count_greater() {
    let input = r#"
fn main() {
    std::crypto::blake2s([true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "blake2s".to_owned(),
            CryptoBlake2sFunction::ARGUMENT_COUNT,
            CryptoBlake2sFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_argument_1_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::blake2s(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s".to_owned(),
            "preimage".to_owned(),
            CryptoBlake2sFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_argument_1_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::blake2s([true; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s".to_owned(),
            "preimage".to_owned(),
            CryptoBlake2sFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_argument_1_preimage_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::crypto::blake2s([true; 4]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s".to_owned(),
            "preimage".to_owned(),
            CryptoBlake2sFunction::ARGUMENT_INDEX_PREIMAGE + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "blake2s_multi_input".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_COUNT,
            CryptoBlake2sMultiInputFunction::ARGUMENT_COUNT - 2,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_count_greater() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true; 8], [true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "blake2s_multi_input".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_COUNT,
            CryptoBlake2sMultiInputFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_1_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input(42, [true;8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_2_preimage_expected_bit_array() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true;8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_1_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true; 0], [true; 8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_multi_input_argument_2_preimage_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true; 8], [true; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_mutli_input_argument_1_preimage_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true; 4], [true; 8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage1".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE1 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_blake2s_mutli_input_argument_2_preimage_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::crypto::blake2s_multi_input([true; 8], [true; 4]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "blake2s_multi_input".to_owned(),
            "preimage2".to_owned(),
            CryptoBlake2sMultiInputFunction::ARGUMENT_INDEX_PREIMAGE2 + 1,
            format!("[bool; N], N > 0, N % {} == 0", crate::BITLENGTH_BYTE),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_count_lesser() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 8];

    Signature::verify(signature);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "verify".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT,
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_count_greater() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 8];

    Signature::verify(signature, message, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "verify".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT,
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_1_signature_expected_signature() {
    let input = r#"
use std::crypto::schnorr::Signature;

fn main() {
    Signature::verify(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(5, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "verify".to_owned(),
            "signature".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_SIGNATURE + 1,
            "std::crypto::schnorr::Signature { r: std::crypto::ecc::Point, s: field, pk: std::crypto::ecc::Point }".to_owned(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_2_message_expected_bit_array() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 8];

    Signature::verify(signature, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "verify".to_owned(),
            "message".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}, N % {} == 0",
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_2_message_expected_bit_array_not_empty() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 0];

    Signature::verify(signature, message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "verify".to_owned(),
            "message".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}, N % {} == 0",
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_2_message_expected_bit_array_size_limit() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 256];

    Signature::verify(signature, message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "verify".to_owned(),
            "message".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}, N % {} == 0",
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(
                Type::boolean(),
                crate::LIMIT_SCHNORR_MESSAGE_BITS + crate::BITLENGTH_BYTE,
            )
            .to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_schnorr_signature_verify_argument_2_message_expected_bit_array_size_multiple_8() {
    let input = r#"
use std::crypto::ecc::Point;
use std::crypto::schnorr::Signature;

fn main() {
    let signature = Signature {
        r: Point { x: 1 as field, y: 2 as field },
        s: 3 as field,
        pk: Point { x: 4 as field, y: 5 as field },
    };
    let message = [true; 4];

    Signature::verify(signature, message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(13, 22),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "verify".to_owned(),
            "message".to_owned(),
            CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
            format!(
                "[bool; N], 0 < N <= {}, N % {} == 0",
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_count_lesser() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_unsigned".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT,
            ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_count_greater() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned([false; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_unsigned".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT,
            ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_1_bits_expected_bit_array() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_unsigned".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_1_bits_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned([false; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_unsigned".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_1_bits_expected_bit_array_size_limit() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned([false; 256]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_unsigned".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(
                Type::boolean(),
                crate::BITLENGTH_MAX_INT + crate::BITLENGTH_BYTE,
            )
            .to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_unsigned_argument_1_bits_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::convert::from_bits_unsigned([false; 4]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 37),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_unsigned".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_count_lesser() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_signed".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_COUNT,
            ConvertFromBitsSignedFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_count_greater() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed([false; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_signed".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_COUNT,
            ConvertFromBitsSignedFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_1_bits_expected_bit_array() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_signed".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_1_bits_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed([false; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_signed".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_1_bits_expected_bit_array_size_limit() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed([false; 256]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_signed".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(
                Type::boolean(),
                crate::BITLENGTH_MAX_INT + crate::BITLENGTH_BYTE,
            )
            .to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_signed_argument_1_bits_expected_bit_array_size_multiple_8() {
    let input = r#"
fn main() {
    std::convert::from_bits_signed([false; 4]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 35),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_signed".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
            format!(
                "[bool; N], {} <= N <= {}, N % {} == 0",
                crate::BITLENGTH_BYTE,
                crate::BITLENGTH_MAX_INT,
                crate::BITLENGTH_BYTE
            ),
            Type::array(Type::boolean(), 4).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_field_argument_count_lesser() {
    let input = r#"
fn main() {
    std::convert::from_bits_field();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 34),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_field".to_owned(),
            ConvertFromBitsFieldFunction::ARGUMENT_COUNT,
            ConvertFromBitsFieldFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_field_argument_count_greater() {
    let input = r#"
fn main() {
    std::convert::from_bits_field([false; 254], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 34),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "from_bits_field".to_owned(),
            ConvertFromBitsFieldFunction::ARGUMENT_COUNT,
            ConvertFromBitsFieldFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_field_argument_1_bits_expected_bit_array() {
    let input = r#"
fn main() {
    std::convert::from_bits_field(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 34),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_field".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
            format!("[bool; {}]", crate::BITLENGTH_FIELD),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_field_argument_1_bits_expected_bit_array_not_empty() {
    let input = r#"
fn main() {
    std::convert::from_bits_field([false; 0]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 34),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_field".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
            format!("[bool; {}]", crate::BITLENGTH_FIELD),
            Type::array(Type::boolean(), 0).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_from_bits_field_argument_1_bits_expected_bit_array_size_field() {
    let input = r#"
fn main() {
    std::convert::from_bits_field([false; 248]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 34),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "from_bits_field".to_owned(),
            "bits".to_owned(),
            ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
            format!("[bool; {}]", crate::BITLENGTH_FIELD),
            Type::array(Type::boolean(), crate::BITLENGTH_MAX_INT).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_to_bits_argument_count_lesser() {
    let input = r#"
fn main() {
    std::convert::to_bits();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "to_bits".to_owned(),
            ConvertToBitsFunction::ARGUMENT_COUNT,
            ConvertToBitsFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_to_bits_argument_count_greater() {
    let input = r#"
fn main() {
    std::convert::to_bits(true, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "to_bits".to_owned(),
            ConvertToBitsFunction::ARGUMENT_COUNT,
            ConvertToBitsFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_convert_to_bits_argument_1_value_expected_scalar() {
    let input = r#"
fn main() {
    std::convert::to_bits((true, false, true, false));
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 26),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "to_bits".to_owned(),
            "value".to_owned(),
            ConvertToBitsFunction::ARGUMENT_INDEX_VALUE + 1,
            "{integer}".to_owned(),
            Type::tuple(vec![Type::boolean(); 4]).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_reverse_argument_count_lesser() {
    let input = r#"
fn main() {
    std::array::reverse();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "reverse".to_owned(),
            ArrayReverseFunction::ARGUMENT_COUNT,
            ArrayReverseFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_reverse_argument_count_greater() {
    let input = r#"
fn main() {
    std::array::reverse([true; 8], 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "reverse".to_owned(),
            ArrayReverseFunction::ARGUMENT_COUNT,
            ArrayReverseFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_reverse_argument_1_array_expected_array() {
    let input = r#"
fn main() {
    std::array::reverse(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 24),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "reverse".to_owned(),
            "array".to_owned(),
            ArrayReverseFunction::ARGUMENT_INDEX_ARRAY + 1,
            "[{scalar}; N]".to_owned(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncate_argument_count_lesser() {
    let input = r#"
fn main() {
    std::array::truncate([true; 8]);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "truncate".to_owned(),
            ArrayTruncateFunction::ARGUMENT_COUNT,
            ArrayTruncateFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncate_argument_count_greater() {
    let input = r#"
fn main() {
    std::array::truncate([true; 8], 4, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "truncate".to_owned(),
            ArrayTruncateFunction::ARGUMENT_COUNT,
            ArrayTruncateFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncate_argument_1_array_expected_array() {
    let input = r#"
fn main() {
    std::array::truncate(42, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "truncate".to_owned(),
            "array".to_owned(),
            ArrayTruncateFunction::ARGUMENT_INDEX_ARRAY + 1,
            "[{scalar}; N]".to_owned(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncate_argument_2_new_length_expected_unsigned_integer() {
    let input = r#"
fn main() {
    std::array::truncate([true; 8], true);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "truncate".to_owned(),
            "new_length".to_owned(),
            ArrayTruncateFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
            "{unsigned integer}".to_owned(),
            Type::boolean().to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncate_argument_2_new_length_expected_unsigned_integer_constant() {
    let input = r#"
fn main() {
    let new_length = 4;
    std::array::truncate([true; 8], new_length);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 25),
        ElementError::Type(TypeError::Function(
            FunctionTypeError::argument_constantness(
                "truncate".to_owned(),
                "new_length".to_owned(),
                ArrayTruncateFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_truncating_to_bigger_size() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::truncate([1, 2], 4)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::StandardLibrary(
            StandardLibraryFunctionTypeError::array_truncating_to_bigger_size(2, 4),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_count_lesser() {
    let input = r#"
fn main() {
    std::array::pad([true; 8], 12);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pad".to_owned(),
            ArrayPadFunction::ARGUMENT_COUNT,
            ArrayPadFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_count_greater() {
    let input = r#"
fn main() {
    std::array::pad([true; 8], 12, false, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "pad".to_owned(),
            ArrayPadFunction::ARGUMENT_COUNT,
            ArrayPadFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_1_array_expected_array() {
    let input = r#"
fn main() {
    std::array::pad(42, 12, false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pad".to_owned(),
            "array".to_owned(),
            ArrayPadFunction::ARGUMENT_INDEX_ARRAY + 1,
            "[{scalar}; N]".to_owned(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_2_new_length_expected_unsigned_integer() {
    let input = r#"
fn main() {
    std::array::pad([true; 8], true, false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pad".to_owned(),
            "new_length".to_owned(),
            ArrayPadFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
            "{unsigned integer}".to_owned(),
            Type::boolean().to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_2_new_length_expected_unsigned_integer_constant() {
    let input = r#"
fn main() {
    let new_length = 4;
    std::array::pad([true; 8], new_length, false);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(4, 20),
        ElementError::Type(TypeError::Function(
            FunctionTypeError::argument_constantness(
                "pad".to_owned(),
                "new_length".to_owned(),
                ArrayPadFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
                Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
            ),
        )),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_pad_argument_3_fill_value_expected_array_element() {
    let input = r#"
fn main() {
    std::array::pad([true; 8], 12, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "pad".to_owned(),
            "fill_value".to_owned(),
            ArrayPadFunction::ARGUMENT_INDEX_FILL_VALUE + 1,
            Type::boolean().to_string(),
            Type::integer_unsigned(crate::BITLENGTH_BYTE).to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_padding_to_lesser_size() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::pad([1, 2, 3, 4], 2, 0)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::StandardLibrary(
            StandardLibraryFunctionTypeError::array_padding_to_lesser_size(4, 2),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_array_new_length_invalid() {
    let input = r#"
fn main() -> [u8; 4] {
    std::array::truncate([1], 0x1_00000000_00000000)
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 25),
        ElementError::Type(TypeError::Function(FunctionTypeError::StandardLibrary(
            StandardLibraryFunctionTypeError::array_new_length_invalid(
                IntegerConstant::new(
                    BigInt::from_str("18446744073709551616")
                        .expect(crate::semantic::tests::PANIC_TEST_DATA),
                    false,
                    72,
                )
                .to_string(),
            ),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_ff_invert_argument_count_lesser() {
    let input = r#"
fn main() {
    std::ff::invert();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "invert".to_owned(),
            FfInvertFunction::ARGUMENT_COUNT,
            FfInvertFunction::ARGUMENT_COUNT - 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_ff_invert_argument_count_greater() {
    let input = r#"
fn main() {
    std::ff::invert(42 as field, true);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_count(
            "invert".to_owned(),
            FfInvertFunction::ARGUMENT_COUNT,
            FfInvertFunction::ARGUMENT_COUNT + 1,
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_ff_invert_argument_1_value_expected_field() {
    let input = r#"
fn main() {
    std::ff::invert(true);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::Element(
        Location::new(3, 20),
        ElementError::Type(TypeError::Function(FunctionTypeError::argument_type(
            "invert".to_owned(),
            "value".to_owned(),
            FfInvertFunction::ARGUMENT_INDEX_VALUE + 1,
            Type::field().to_string(),
            Type::boolean().to_string(),
        ))),
    )));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
