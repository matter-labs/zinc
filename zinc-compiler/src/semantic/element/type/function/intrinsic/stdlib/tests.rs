//!
//! The standard library intrinsic functions tests.
//!

use std::str::FromStr;

use num::BigInt;

use zinc_lexical::Keyword;
use zinc_lexical::Location;

use crate::error::Error;
use crate::semantic::element::constant::integer::Integer as IntegerConstant;
use crate::semantic::element::r#type::function::intrinsic::stdlib::collections_mtreemap_get::Function as CollectionsMTreeMapGetFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::collections_mtreemap_contains::Function as CollectionsMTreeMapContainsFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::collections_mtreemap_insert::Function as CollectionsMTreeMapInsertFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::collections_mtreemap_remove::Function as CollectionsMTreeMapRemoveFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::array_pad::Function as ArrayPadFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::array_reverse::Function as ArrayReverseFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::array_truncate::Function as ArrayTruncateFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::convert_from_bits_field::Function as ConvertFromBitsFieldFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::convert_from_bits_signed::Function as ConvertFromBitsSignedFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::convert_from_bits_unsigned::Function as ConvertFromBitsUnsignedFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::convert_to_bits::Function as ConvertToBitsFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::crypto_pedersen::Function as CryptoPedersenFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::crypto_schnorr_signature_verify::Function as CryptoSchnorrSignatureVerifyFunction;
use crate::semantic::element::r#type::function::intrinsic::stdlib::crypto_sha256::Function as CryptoSha256Function;
use crate::semantic::element::r#type::function::intrinsic::stdlib::ff_invert::Function as FfInvertFunction;
use crate::semantic::element::r#type::Type;
use crate::semantic::error::Error as SemanticError;

#[test]
fn error_calling_mutable_from_immutable_contract_mtreemap_insert() {
    let input = r#"
use std::collections::MTreeMap;

contract Data {
    data: MTreeMap<u8, field>;

    pub fn immutable(self) {
        self.data.insert(42, 25 as field);
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionCallMutableFromImmutable {
            location: Location::test(8, 25),
            function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_calling_mutable_from_immutable_contract_mtreemap_remove() {
    let input = r#"
use std::collections::MTreeMap;

contract Data {
    data: MTreeMap<u8, field>;

    pub fn immutable(self) {
        self.data.remove(42);
    }
}
"#;

    let expected = Err(Error::Semantic(
        SemanticError::FunctionCallMutableFromImmutable {
            location: Location::test(8, 25),
            function: CollectionsMTreeMapRemoveFunction::IDENTIFIER.to_owned(),
        },
    ));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_crypto_sha256_argument_count_lesser() {
    let input = r#"
fn main() {
    std::crypto::sha256();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: CryptoSha256Function::IDENTIFIER.to_owned(),
        expected: CryptoSha256Function::ARGUMENT_COUNT,
        found: CryptoSha256Function::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: CryptoSha256Function::IDENTIFIER.to_owned(),
        expected: CryptoSha256Function::ARGUMENT_COUNT,
        found: CryptoSha256Function::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 25),
        function: CryptoSha256Function::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!("[bool; N], N > 0, N % {} == 0", zinc_const::bitlength::BYTE),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 25),
        function: CryptoSha256Function::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!("[bool; N], N > 0, N % {} == 0", zinc_const::bitlength::BYTE),
        found: Type::array(Some(Location::test(3, 25)), Type::boolean(None), 0).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 25),
        function: CryptoSha256Function::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoSha256Function::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!("[bool; N], N > 0, N % {} == 0", zinc_const::bitlength::BYTE),
        found: Type::array(Some(Location::test(3, 25)), Type::boolean(None), 4).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: CryptoPedersenFunction::IDENTIFIER.to_owned(),
        expected: CryptoPedersenFunction::ARGUMENT_COUNT,
        found: CryptoPedersenFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: CryptoPedersenFunction::IDENTIFIER.to_owned(),
        expected: CryptoPedersenFunction::ARGUMENT_COUNT,
        found: CryptoPedersenFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 27),
        function: CryptoPedersenFunction::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}",
            zinc_const::limit::PEDERSEN_HASH_INPUT_BITS
        ),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 27),
        function: CryptoPedersenFunction::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}",
            zinc_const::limit::PEDERSEN_HASH_INPUT_BITS
        ),
        found: Type::array(Some(Location::test(3, 27)), Type::boolean(None), 0).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 27),
        function: CryptoPedersenFunction::IDENTIFIER.to_owned(),
        name: "preimage".to_owned(),
        position: CryptoPedersenFunction::ARGUMENT_INDEX_PREIMAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}",
            zinc_const::limit::PEDERSEN_HASH_INPUT_BITS
        ),
        found: Type::array(
            Some(Location::test(3, 27)),
            Type::boolean(None),
            zinc_const::limit::PEDERSEN_HASH_INPUT_BITS + 1,
        )
        .to_string(),
    }));

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

    signature.verify();
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(13, 21),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        expected: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT,
        found: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    signature.verify(message, 42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(13, 21),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        expected: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT,
        found: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
            location: Location::test(5, 23),
            function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
            name: "signature".to_owned(),
            position: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_SIGNATURE + 1,
            expected: "std::crypto::schnorr::Signature { r: std::crypto::ecc::Point, s: field, pk: std::crypto::ecc::Point }".to_owned(),
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        },
    ));

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

    signature.verify(42);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(13, 22),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        name: "message".to_owned(),
        position: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}, N % {} == 0",
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    signature.verify(message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(13, 22),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        name: "message".to_owned(),
        position: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}, N % {} == 0",
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(13, 34)), Type::boolean(None), 0).to_string(),
    }));

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

    signature.verify(message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(13, 22),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        name: "message".to_owned(),
        position: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}, N % {} == 0",
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(
            Some(Location::test(13, 34)),
            Type::boolean(None),
            zinc_const::limit::SCHNORR_MESSAGE_BITS + zinc_const::bitlength::BYTE,
        )
        .to_string(),
    }));

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

    signature.verify(message);
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(13, 22),
        function: CryptoSchnorrSignatureVerifyFunction::IDENTIFIER.to_owned(),
        name: "message".to_owned(),
        position: CryptoSchnorrSignatureVerifyFunction::ARGUMENT_INDEX_MESSAGE + 1,
        expected: format!(
            "[bool; N], 0 < N <= {}, N % {} == 0",
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(13, 34)), Type::boolean(None), 4).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsUnsignedFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 38),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 38),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(3, 38)), Type::boolean(None), 0).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 38),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(
            Some(Location::test(3, 38)),
            Type::boolean(None),
            zinc_const::bitlength::INTEGER_MAX + zinc_const::bitlength::BYTE,
        )
        .to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 38),
        function: ConvertFromBitsUnsignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsUnsignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(3, 38)), Type::boolean(None), 4).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsSignedFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsSignedFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsSignedFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsSignedFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 36),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 36),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(3, 36)), Type::boolean(None), 0).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 36),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(
            Some(Location::test(3, 36)),
            Type::boolean(None),
            zinc_const::bitlength::INTEGER_MAX + zinc_const::bitlength::BYTE,
        )
        .to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 36),
        function: ConvertFromBitsSignedFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsSignedFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!(
            "[bool; N], {} <= N <= {}, N % {} == 0",
            zinc_const::bitlength::BYTE,
            zinc_const::bitlength::INTEGER_MAX,
            zinc_const::bitlength::BYTE
        ),
        found: Type::array(Some(Location::test(3, 36)), Type::boolean(None), 4).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsFieldFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsFieldFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsFieldFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertFromBitsFieldFunction::IDENTIFIER.to_owned(),
        expected: ConvertFromBitsFieldFunction::ARGUMENT_COUNT,
        found: ConvertFromBitsFieldFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 35),
        function: ConvertFromBitsFieldFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!("[bool; {}]", zinc_const::bitlength::FIELD),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 35),
        function: ConvertFromBitsFieldFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!("[bool; {}]", zinc_const::bitlength::FIELD),
        found: Type::array(Some(Location::test(3, 35)), Type::boolean(None), 0).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 35),
        function: ConvertFromBitsFieldFunction::IDENTIFIER.to_owned(),
        name: "bits".to_owned(),
        position: ConvertFromBitsFieldFunction::ARGUMENT_INDEX_BITS + 1,
        expected: format!("[bool; {}]", zinc_const::bitlength::FIELD),
        found: Type::array(
            Some(Location::test(3, 35)),
            Type::boolean(None),
            zinc_const::bitlength::INTEGER_MAX,
        )
        .to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertToBitsFunction::IDENTIFIER.to_owned(),
        expected: ConvertToBitsFunction::ARGUMENT_COUNT,
        found: ConvertToBitsFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ConvertToBitsFunction::IDENTIFIER.to_owned(),
        expected: ConvertToBitsFunction::ARGUMENT_COUNT,
        found: ConvertToBitsFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 27),
        function: ConvertToBitsFunction::IDENTIFIER.to_owned(),
        name: "value".to_owned(),
        position: ConvertToBitsFunction::ARGUMENT_INDEX_VALUE + 1,
        expected: "{integer}".to_owned(),
        found: Type::tuple(Some(Location::test(3, 27)), vec![Type::boolean(None); 4]).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayReverseFunction::IDENTIFIER.to_owned(),
        expected: ArrayReverseFunction::ARGUMENT_COUNT,
        found: ArrayReverseFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayReverseFunction::IDENTIFIER.to_owned(),
        expected: ArrayReverseFunction::ARGUMENT_COUNT,
        found: ArrayReverseFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 25),
        function: ArrayReverseFunction::IDENTIFIER.to_owned(),
        name: "array".to_owned(),
        position: ArrayReverseFunction::ARGUMENT_INDEX_ARRAY + 1,
        expected: "[{scalar}; N]".to_owned(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayTruncateFunction::IDENTIFIER.to_owned(),
        expected: ArrayTruncateFunction::ARGUMENT_COUNT,
        found: ArrayTruncateFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayTruncateFunction::IDENTIFIER.to_owned(),
        expected: ArrayTruncateFunction::ARGUMENT_COUNT,
        found: ArrayTruncateFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 26),
        function: ArrayTruncateFunction::IDENTIFIER.to_owned(),
        name: "array".to_owned(),
        position: ArrayTruncateFunction::ARGUMENT_INDEX_ARRAY + 1,
        expected: "[{scalar}; N]".to_owned(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 37),
        function: ArrayTruncateFunction::IDENTIFIER.to_owned(),
        name: "new_length".to_owned(),
        position: ArrayTruncateFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
        expected: "{unsigned integer}".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

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

    let expected = Err(Error::Semantic(
        SemanticError::FunctionArgumentConstantness {
            location: Location::test(4, 37),
            function: ArrayTruncateFunction::IDENTIFIER.to_owned(),
            name: "new_length".to_owned(),
            position: ArrayTruncateFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::FunctionStdlibArrayTruncatingToBiggerSize {
            location: Location::test(3, 5),
            from: 2,
            to: 4,
        },
    ));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayPadFunction::IDENTIFIER.to_owned(),
        expected: ArrayPadFunction::ARGUMENT_COUNT,
        found: ArrayPadFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: ArrayPadFunction::IDENTIFIER.to_owned(),
        expected: ArrayPadFunction::ARGUMENT_COUNT,
        found: ArrayPadFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 21),
        function: ArrayPadFunction::IDENTIFIER.to_owned(),
        name: "array".to_owned(),
        position: ArrayPadFunction::ARGUMENT_INDEX_ARRAY + 1,
        expected: "[{scalar}; N]".to_owned(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 32),
        function: ArrayPadFunction::IDENTIFIER.to_owned(),
        name: "new_length".to_owned(),
        position: ArrayPadFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
        expected: "{unsigned integer}".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

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

    let expected = Err(Error::Semantic(
        SemanticError::FunctionArgumentConstantness {
            location: Location::test(4, 32),
            function: ArrayPadFunction::IDENTIFIER.to_owned(),
            name: "new_length".to_owned(),
            position: ArrayPadFunction::ARGUMENT_INDEX_NEW_LENGTH + 1,
            found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 36),
        function: ArrayPadFunction::IDENTIFIER.to_owned(),
        name: "fill_value".to_owned(),
        position: ArrayPadFunction::ARGUMENT_INDEX_FILL_VALUE + 1,
        expected: Type::boolean(None).to_string(),
        found: Type::integer_unsigned(None, zinc_const::bitlength::BYTE).to_string(),
    }));

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

    let expected = Err(Error::Semantic(
        SemanticError::FunctionStdlibArrayPaddingToLesserSize {
            location: Location::test(3, 5),
            from: 4,
            to: 2,
        },
    ));

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

    let expected = Err(Error::Semantic(
        SemanticError::FunctionStdlibArrayNewLengthInvalid {
            location: Location::test(3, 31),
            value: IntegerConstant::new(
                Location::test(3, 31),
                BigInt::from_str("18446744073709551616").expect(zinc_const::panic::TEST_DATA_VALID),
                false,
                72,
                true,
            )
            .to_string(),
        },
    ));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: FfInvertFunction::IDENTIFIER.to_owned(),
        expected: FfInvertFunction::ARGUMENT_COUNT,
        found: FfInvertFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(3, 5),
        function: FfInvertFunction::IDENTIFIER.to_owned(),
        expected: FfInvertFunction::ARGUMENT_COUNT,
        found: FfInvertFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

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

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(3, 21),
        function: FfInvertFunction::IDENTIFIER.to_owned(),
        name: "value".to_owned(),
        position: FfInvertFunction::ARGUMENT_INDEX_VALUE + 1,
        expected: Type::field(None).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_get_argument_count_lesser() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.get();
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 24),
        function: CollectionsMTreeMapGetFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapGetFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapGetFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_get_argument_count_greater() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.get(0 as u160, true);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 24),
        function: CollectionsMTreeMapGetFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapGetFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapGetFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_get_argument_1_self_expected_map() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        MTreeMap::get(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 23),
        function: CollectionsMTreeMapGetFunction::IDENTIFIER.to_owned(),
        name: Keyword::SelfLowercase.to_string(),
        position: CollectionsMTreeMapGetFunction::ARGUMENT_INDEX_SELF + 1,
        expected: "std::collections::MTreeMap".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_get_argument_2_key_expected_u160() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.get(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 25),
        function: CollectionsMTreeMapGetFunction::IDENTIFIER.to_owned(),
        name: "key".to_owned(),
        position: CollectionsMTreeMapGetFunction::ARGUMENT_INDEX_KEY + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_contains_argument_count_lesser() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.contains();
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 29),
        function: CollectionsMTreeMapContainsFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapContainsFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapContainsFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_contains_argument_count_greater() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.contains(0 as u160, true);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 29),
        function: CollectionsMTreeMapContainsFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapContainsFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapContainsFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_contains_argument_1_self_expected_map() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        MTreeMap::contains(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 28),
        function: CollectionsMTreeMapContainsFunction::IDENTIFIER.to_owned(),
        name: Keyword::SelfLowercase.to_string(),
        position: CollectionsMTreeMapContainsFunction::ARGUMENT_INDEX_SELF + 1,
        expected: "std::collections::MTreeMap".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_contains_argument_2_key_expected_u160() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(self) -> u248 {
        self.values.contains(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 30),
        function: CollectionsMTreeMapContainsFunction::IDENTIFIER.to_owned(),
        name: "key".to_owned(),
        position: CollectionsMTreeMapContainsFunction::ARGUMENT_INDEX_KEY + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_insert_argument_count_lesser() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.insert(0x0000000000000000000000000000000000000000 as u160);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 27),
        function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapInsertFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapInsertFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_insert_argument_count_greater() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.insert(0 as u160, 1000 as u248, false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 27),
        function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapInsertFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapInsertFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_insert_argument_1_self_expected_map() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        MTreeMap::insert(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 26),
        function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        name: Keyword::SelfLowercase.to_string(),
        position: CollectionsMTreeMapInsertFunction::ARGUMENT_INDEX_SELF + 1,
        expected: "std::collections::MTreeMap".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_insert_argument_2_key_expected_u160() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.insert(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 28),
        function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        name: "key".to_owned(),
        position: CollectionsMTreeMapInsertFunction::ARGUMENT_INDEX_KEY + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_insert_argument_3_value_expected_u248() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.insert(0x0000000000000000000000000000000000000000 as u160, false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 80),
        function: CollectionsMTreeMapInsertFunction::IDENTIFIER.to_owned(),
        name: "value".to_owned(),
        position: CollectionsMTreeMapInsertFunction::ARGUMENT_INDEX_VALUE + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::BALANCE).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_remove_argument_count_lesser() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.remove();
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 27),
        function: CollectionsMTreeMapRemoveFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapRemoveFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapRemoveFunction::ARGUMENT_COUNT - 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_remove_argument_count_greater() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.remove(0 as u160, true);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentCount {
        location: Location::test(8, 27),
        function: CollectionsMTreeMapRemoveFunction::IDENTIFIER.to_owned(),
        expected: CollectionsMTreeMapRemoveFunction::ARGUMENT_COUNT,
        found: CollectionsMTreeMapRemoveFunction::ARGUMENT_COUNT + 1,
        reference: None,
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_remove_argument_1_self_expected_map() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        MTreeMap::remove(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 26),
        function: CollectionsMTreeMapRemoveFunction::IDENTIFIER.to_owned(),
        name: Keyword::SelfLowercase.to_string(),
        position: CollectionsMTreeMapRemoveFunction::ARGUMENT_INDEX_SELF + 1,
        expected: "std::collections::MTreeMap".to_owned(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}

#[test]
fn error_collections_mtreemap_remove_argument_2_key_expected_u160() {
    let input = r#"
use std::collections::MTreeMap;

contract Test {
    values: MTreeMap<u160, u248>;

    pub fn test(mut self) -> u248 {
        self.values.remove(false);
    }
}
"#;

    let expected = Err(Error::Semantic(SemanticError::FunctionArgumentType {
        location: Location::test(8, 28),
        function: CollectionsMTreeMapRemoveFunction::IDENTIFIER.to_owned(),
        name: "key".to_owned(),
        position: CollectionsMTreeMapRemoveFunction::ARGUMENT_INDEX_KEY + 1,
        expected: Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS).to_string(),
        found: Type::boolean(None).to_string(),
    }));

    let result = crate::semantic::tests::compile_entry(input);

    assert_eq!(result, expected);
}
