//!
//! The semantic analyzer scope built-in items.
//!

use zinc_bytecode::builtins::BuiltinIdentifier;

use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::module::Module as ScopeModuleItem;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

///
/// A built-in items set instance creator.
///
/// The built-in items are the built-in functions `dbg!` and `assert!` and the standard library.
///
#[derive(Debug)]
pub struct BuiltInScope {}

pub enum BuiltInTypeId {
    StdCryptoEccPoint = 0,
    StdCryptoSchnorrSignature = 1,
}

impl BuiltInScope {
    pub fn initialize() -> Scope {
        let std_scope = Scope::new_built_in("std");

        std_scope.items.borrow_mut().insert(
            "crypto".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "crypto".to_owned(),
                Self::module_crypto().wrap(),
            ))
            .wrap(),
        );
        std_scope.items.borrow_mut().insert(
            "convert".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "convert".to_owned(),
                Self::module_convert().wrap(),
            ))
            .wrap(),
        );
        std_scope.items.borrow_mut().insert(
            "array".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "array".to_owned(),
                Self::module_array().wrap(),
            ))
            .wrap(),
        );
        std_scope.items.borrow_mut().insert(
            "ff".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "ff".to_owned(),
                Self::module_ff().wrap(),
            ))
            .wrap(),
        );

        let scope = Scope::new_built_in("root");

        let builtin_function_dbg = FunctionType::new_dbg();
        scope.items.borrow_mut().insert(
            builtin_function_dbg.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                builtin_function_dbg,
            )))
            .wrap(),
        );

        let builtin_function_assert = FunctionType::new_assert();
        scope.items.borrow_mut().insert(
            builtin_function_assert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                builtin_function_assert,
            )))
            .wrap(),
        );

        scope.items.borrow_mut().insert(
            "std".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "std".to_owned(),
                std_scope.wrap(),
            ))
            .wrap(),
        );

        scope
    }

    fn module_crypto() -> Scope {
        let std_crypto_scope = Scope::new_built_in("crypto");

        let std_crypto_sha256 = FunctionType::new_std(BuiltinIdentifier::CryptoSha256);
        let std_crypto_pedersen = FunctionType::new_std(BuiltinIdentifier::CryptoPedersen);

        let std_crypto_schnorr_scope = Scope::new_built_in("schnorr");
        let std_crypto_schnorr_signature_scope = Scope::new_built_in("Signature");
        let std_crypto_schnorr_verify =
            FunctionType::new_std(BuiltinIdentifier::CryptoSchnorrSignatureVerify);
        std_crypto_schnorr_signature_scope
            .items
            .borrow_mut()
            .insert(
                std_crypto_schnorr_verify.identifier(),
                ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                    std_crypto_schnorr_verify,
                )))
                .wrap(),
            );
        let std_crypto_ecc_point = StructureType::new(
            None,
            "Point".to_owned(),
            BuiltInTypeId::StdCryptoEccPoint as usize,
            vec![
                ("x".to_owned(), Type::field(None)),
                ("y".to_owned(), Type::field(None)),
            ],
            None,
        );
        let std_crypto_schnorr_signature = StructureType::new(
            None,
            "Signature".to_owned(),
            BuiltInTypeId::StdCryptoSchnorrSignature as usize,
            vec![
                (
                    "r".to_owned(),
                    Type::Structure(std_crypto_ecc_point.clone()),
                ),
                ("s".to_owned(), Type::field(None)),
                (
                    "pk".to_owned(),
                    Type::Structure(std_crypto_ecc_point.clone()),
                ),
            ],
            Some(std_crypto_schnorr_signature_scope.wrap()),
        );
        std_crypto_schnorr_scope.items.borrow_mut().insert(
            "Signature".to_owned(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(
                std_crypto_schnorr_signature,
            )))
            .wrap(),
        );

        let std_crypto_ecc_scope = Scope::new_built_in("ecc");
        std_crypto_ecc_scope.items.borrow_mut().insert(
            "Point".to_owned(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(
                std_crypto_ecc_point,
            )))
            .wrap(),
        );

        std_crypto_scope.items.borrow_mut().insert(
            std_crypto_sha256.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_crypto_sha256,
            )))
            .wrap(),
        );
        std_crypto_scope.items.borrow_mut().insert(
            std_crypto_pedersen.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_crypto_pedersen,
            )))
            .wrap(),
        );
        std_crypto_scope.items.borrow_mut().insert(
            "ecc".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "ecc".to_owned(),
                std_crypto_ecc_scope.wrap(),
            ))
            .wrap(),
        );
        std_crypto_scope.items.borrow_mut().insert(
            "schnorr".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "schnorr".to_owned(),
                std_crypto_schnorr_scope.wrap(),
            ))
            .wrap(),
        );

        std_crypto_scope
    }

    fn module_convert() -> Scope {
        let std_convert_scope = Scope::new_built_in("convert");

        let std_convert_to_bits = FunctionType::new_std(BuiltinIdentifier::ToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_std(BuiltinIdentifier::UnsignedFromBits);
        let std_convert_from_bits_signed = FunctionType::new_std(BuiltinIdentifier::SignedFromBits);
        let std_convert_from_bits_field = FunctionType::new_std(BuiltinIdentifier::FieldFromBits);

        std_convert_scope.items.borrow_mut().insert(
            std_convert_to_bits.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_convert_to_bits,
            )))
            .wrap(),
        );
        std_convert_scope.items.borrow_mut().insert(
            std_convert_from_bits_unsigned.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_convert_from_bits_unsigned,
            )))
            .wrap(),
        );
        std_convert_scope.items.borrow_mut().insert(
            std_convert_from_bits_signed.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_convert_from_bits_signed,
            )))
            .wrap(),
        );
        std_convert_scope.items.borrow_mut().insert(
            std_convert_from_bits_field.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_convert_from_bits_field,
            )))
            .wrap(),
        );

        std_convert_scope
    }

    fn module_array() -> Scope {
        let std_array_scope = Scope::new_built_in("array");

        let std_array_reverse = FunctionType::new_std(BuiltinIdentifier::ArrayReverse);
        let std_array_truncate = FunctionType::new_std(BuiltinIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_std(BuiltinIdentifier::ArrayPad);

        std_array_scope.items.borrow_mut().insert(
            std_array_reverse.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_array_reverse,
            )))
            .wrap(),
        );
        std_array_scope.items.borrow_mut().insert(
            std_array_truncate.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                std_array_truncate,
            )))
            .wrap(),
        );
        std_array_scope.items.borrow_mut().insert(
            std_array_pad.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(std_array_pad))).wrap(),
        );

        std_array_scope
    }

    fn module_ff() -> Scope {
        let std_ff_scope = Scope::new_built_in("ff");

        let std_ff_invert = FunctionType::new_std(BuiltinIdentifier::FieldInverse);

        std_ff_scope.items.borrow_mut().insert(
            std_ff_invert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(std_ff_invert))).wrap(),
        );

        std_ff_scope
    }
}
