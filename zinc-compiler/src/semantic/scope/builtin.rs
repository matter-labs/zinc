//!
//! The semantic analyzer scope built-in items.
//!

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::variant::Variant as ScopeItemVariant;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

///
/// A built-in items set instance creator.
///
/// The built-in items are the built-in functions `dbg!` and `assert!` and the standard library.
///
#[derive(Debug)]
pub struct BuiltInItems {}

impl BuiltInItems {
    pub const TYPE_ID_STD_CRYPTO_ECC_POINT: usize = 0;
    pub const TYPE_ID_STD_CRYPTO_SCHNORR_SIGNATURE: usize = 1;
    pub const TYPE_ID_FIRST_AVAILABLE: usize = 2;

    pub fn new_map() -> HashMap<String, ScopeItem> {
        let mut std_crypto_scope = Scope::default();
        let std_crypto_sha256 = FunctionType::new_std(BuiltinIdentifier::CryptoSha256);
        let std_crypto_pedersen = FunctionType::new_std(BuiltinIdentifier::CryptoPedersen);
        let std_crypto_blake2s = FunctionType::new_std(BuiltinIdentifier::CryptoBlake2s);
        let std_crypto_blake2s_multi_input =
            FunctionType::new_std(BuiltinIdentifier::CryptoBlake2sMultiInput);

        let mut std_crypto_schnorr = Scope::default();
        let mut std_crypto_schnorr_signature_scope = Scope::default();
        let std_crypto_schnorr_verify =
            FunctionType::new_std(BuiltinIdentifier::CryptoSchnorrSignatureVerify);
        std_crypto_schnorr_signature_scope.items.insert(
            std_crypto_schnorr_verify.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_crypto_schnorr_verify)),
                None,
            ),
        );
        let std_crypto_ecc_point = StructureType::new(
            "Point".to_owned(),
            Self::TYPE_ID_STD_CRYPTO_ECC_POINT,
            vec![
                ("x".to_owned(), Type::field()),
                ("y".to_owned(), Type::field()),
            ],
            None,
        );
        let std_crypto_schnorr_signature = StructureType::new(
            "Signature".to_owned(),
            Self::TYPE_ID_STD_CRYPTO_SCHNORR_SIGNATURE,
            vec![
                (
                    "r".to_owned(),
                    Type::Structure(std_crypto_ecc_point.clone()),
                ),
                ("s".to_owned(), Type::field()),
                (
                    "pk".to_owned(),
                    Type::Structure(std_crypto_ecc_point.clone()),
                ),
            ],
            Some(Rc::new(RefCell::new(std_crypto_schnorr_signature_scope))),
        );
        std_crypto_schnorr.items.insert(
            "Signature".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Structure(std_crypto_schnorr_signature)),
                None,
            ),
        );

        let mut std_crypto_ecc = Scope::default();
        std_crypto_ecc.items.insert(
            "Point".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Structure(std_crypto_ecc_point)),
                None,
            ),
        );

        std_crypto_scope.items.insert(
            std_crypto_sha256.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_crypto_sha256)),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            std_crypto_pedersen.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_crypto_pedersen)),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            std_crypto_blake2s.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_crypto_blake2s)),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            std_crypto_blake2s_multi_input.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_crypto_blake2s_multi_input)),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            "ecc".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_crypto_ecc))),
                None,
            ),
        );
        std_crypto_scope.items.insert(
            "schnorr".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_crypto_schnorr))),
                None,
            ),
        );

        let mut std_convert_scope = Scope::default();
        let std_convert_to_bits = FunctionType::new_std(BuiltinIdentifier::ToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_std(BuiltinIdentifier::UnsignedFromBits);
        let std_convert_from_bits_signed = FunctionType::new_std(BuiltinIdentifier::SignedFromBits);
        let std_convert_from_bits_field = FunctionType::new_std(BuiltinIdentifier::FieldFromBits);
        std_convert_scope.items.insert(
            std_convert_to_bits.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_convert_to_bits)),
                None,
            ),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_unsigned.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_convert_from_bits_unsigned)),
                None,
            ),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_signed.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_convert_from_bits_signed)),
                None,
            ),
        );
        std_convert_scope.items.insert(
            std_convert_from_bits_field.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_convert_from_bits_field)),
                None,
            ),
        );

        let mut std_array_scope = Scope::default();
        let std_array_reverse = FunctionType::new_std(BuiltinIdentifier::ArrayReverse);
        let std_array_truncate = FunctionType::new_std(BuiltinIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_std(BuiltinIdentifier::ArrayPad);
        std_array_scope.items.insert(
            std_array_reverse.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_array_reverse)),
                None,
            ),
        );
        std_array_scope.items.insert(
            std_array_truncate.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(std_array_truncate)),
                None,
            ),
        );
        std_array_scope.items.insert(
            std_array_pad.identifier(),
            ScopeItem::new(ScopeItemVariant::Type(Type::Function(std_array_pad)), None),
        );

        let mut std_ff_scope = Scope::default();
        let std_ff_invert = FunctionType::new_std(BuiltinIdentifier::FieldInverse);
        std_ff_scope.items.insert(
            std_ff_invert.identifier(),
            ScopeItem::new(ScopeItemVariant::Type(Type::Function(std_ff_invert)), None),
        );

        let mut std_scope = Scope::default();
        std_scope.items.insert(
            "crypto".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_crypto_scope))),
                None,
            ),
        );
        std_scope.items.insert(
            "convert".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_convert_scope))),
                None,
            ),
        );
        std_scope.items.insert(
            "array".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_array_scope))),
                None,
            ),
        );
        std_scope.items.insert(
            "ff".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_ff_scope))),
                None,
            ),
        );

        let mut items = HashMap::with_capacity(3);
        let builtin_function_dbg = FunctionType::new_dbg();
        let builtin_function_assert = FunctionType::new_assert();
        items.insert(
            builtin_function_dbg.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(builtin_function_dbg)),
                None,
            ),
        );
        items.insert(
            builtin_function_assert.identifier(),
            ScopeItem::new(
                ScopeItemVariant::Type(Type::Function(builtin_function_assert)),
                None,
            ),
        );
        items.insert(
            "std".to_owned(),
            ScopeItem::new(
                ScopeItemVariant::Module(Rc::new(RefCell::new(std_scope))),
                None,
            ),
        );
        items
    }
}
