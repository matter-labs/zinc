//!
//! The semantic analyzer scope built-in items.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::FunctionIdentifier;

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

///
/// The built-in structures type IDs.
///
pub enum BuiltInTypeId {
    /// The `std::crypto::ecc::Point` structure type ID.
    StdCryptoEccPoint = 0,
    /// The `std::crypto::schnorr::Signature` structure type ID.
    StdCryptoSchnorrSignature = 1,
    /// The `std::assets::Token` structure type ID.
    StdAssetsToken = 2,
}

impl BuiltInScope {
    ///
    /// Initializes the built-in module scope.
    ///
    pub fn initialize() -> Rc<RefCell<Scope>> {
        let std_scope = Scope::new_built_in("std").wrap();

        Scope::insert_item(
            std_scope.clone(),
            "crypto".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "crypto".to_owned(),
                Self::module_crypto(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_scope.clone(),
            "convert".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "convert".to_owned(),
                Self::module_convert(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_scope.clone(),
            "array".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "array".to_owned(),
                Self::module_array(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_scope.clone(),
            "ff".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "ff".to_owned(),
                Self::module_ff(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_scope.clone(),
            "assets".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "assets".to_owned(),
                Self::module_assets(),
            ))
            .wrap(),
        );

        let root_scope = Scope::new_built_in("root").wrap();

        let builtin_function_dbg = FunctionType::new_dbg();
        Scope::insert_item(
            root_scope.clone(),
            builtin_function_dbg.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(builtin_function_dbg),
                false,
            ))
            .wrap(),
        );

        let builtin_function_assert = FunctionType::new_assert();
        Scope::insert_item(
            root_scope.clone(),
            builtin_function_assert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(builtin_function_assert),
                false,
            ))
            .wrap(),
        );

        Scope::insert_item(
            root_scope.clone(),
            "std".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in("std".to_owned(), std_scope)).wrap(),
        );

        root_scope
    }

    ///
    /// Initializes the `std::crypto` module scope.
    ///
    fn module_crypto() -> Rc<RefCell<Scope>> {
        let std_crypto_scope = Scope::new_built_in("crypto").wrap();

        let std_crypto_sha256 = FunctionType::new_std(FunctionIdentifier::CryptoSha256);
        let std_crypto_pedersen = FunctionType::new_std(FunctionIdentifier::CryptoPedersen);

        let std_crypto_schnorr_scope = Scope::new_built_in("schnorr").wrap();
        let std_crypto_schnorr_signature_scope = Scope::new_built_in("Signature").wrap();
        let std_crypto_schnorr_verify =
            FunctionType::new_std(FunctionIdentifier::CryptoSchnorrSignatureVerify);
        Scope::insert_item(
            std_crypto_schnorr_signature_scope.clone(),
            std_crypto_schnorr_verify.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_crypto_schnorr_verify),
                true,
            ))
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
            std_crypto_schnorr_signature_scope.borrow().name(),
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
            Some(std_crypto_schnorr_signature_scope.clone()),
        );
        Scope::insert_item(
            std_crypto_schnorr_scope.clone(),
            std_crypto_schnorr_signature_scope.borrow().name(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Structure(std_crypto_schnorr_signature),
                false,
            ))
            .wrap(),
        );

        let std_crypto_ecc_scope = Scope::new_built_in("ecc").wrap();
        Scope::insert_item(
            std_crypto_ecc_scope.clone(),
            std_crypto_ecc_point.identifier.clone(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Structure(std_crypto_ecc_point),
                false,
            ))
            .wrap(),
        );

        Scope::insert_item(
            std_crypto_scope.clone(),
            std_crypto_sha256.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_crypto_sha256),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_crypto_scope.clone(),
            std_crypto_pedersen.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_crypto_pedersen),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_crypto_scope.clone(),
            std_crypto_ecc_scope.borrow().name(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                std_crypto_ecc_scope.borrow().name(),
                std_crypto_ecc_scope.clone(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_crypto_scope.clone(),
            std_crypto_schnorr_scope.borrow().name(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                std_crypto_schnorr_scope.borrow().name(),
                std_crypto_schnorr_scope.clone(),
            ))
            .wrap(),
        );

        std_crypto_scope
    }

    ///
    /// Initializes the `std::convert` module scope.
    ///
    fn module_convert() -> Rc<RefCell<Scope>> {
        let std_convert_scope = Scope::new_built_in("convert").wrap();

        let std_convert_to_bits = FunctionType::new_std(FunctionIdentifier::ConvertToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_std(FunctionIdentifier::ConvertFromBitsUnsigned);
        let std_convert_from_bits_signed =
            FunctionType::new_std(FunctionIdentifier::ConvertFromBitsSigned);
        let std_convert_from_bits_field =
            FunctionType::new_std(FunctionIdentifier::ConvertFromBitsField);

        Scope::insert_item(
            std_convert_scope.clone(),
            std_convert_to_bits.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_convert_to_bits),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_convert_scope.clone(),
            std_convert_from_bits_unsigned.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_convert_from_bits_unsigned),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_convert_scope.clone(),
            std_convert_from_bits_signed.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_convert_from_bits_signed),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_convert_scope.clone(),
            std_convert_from_bits_field.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_convert_from_bits_field),
                false,
            ))
            .wrap(),
        );

        std_convert_scope
    }

    ///
    /// Initializes the `std::array` module scope.
    ///
    fn module_array() -> Rc<RefCell<Scope>> {
        let std_array_scope = Scope::new_built_in("array").wrap();

        let std_array_reverse = FunctionType::new_std(FunctionIdentifier::ArrayReverse);
        let std_array_truncate = FunctionType::new_std(FunctionIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_std(FunctionIdentifier::ArrayPad);

        Scope::insert_item(
            std_array_scope.clone(),
            std_array_reverse.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_array_reverse),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_array_scope.clone(),
            std_array_truncate.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_array_truncate),
                false,
            ))
            .wrap(),
        );
        Scope::insert_item(
            std_array_scope.clone(),
            std_array_pad.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_array_pad),
                false,
            ))
            .wrap(),
        );

        std_array_scope
    }

    ///
    /// Initializes the `std::ff` module scope.
    ///
    fn module_ff() -> Rc<RefCell<Scope>> {
        let std_ff_scope = Scope::new_built_in("ff").wrap();

        let std_ff_invert = FunctionType::new_std(FunctionIdentifier::FieldInverse);

        Scope::insert_item(
            std_ff_scope.clone(),
            std_ff_invert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_ff_invert),
                false,
            ))
            .wrap(),
        );

        std_ff_scope
    }

    ///
    /// Initializes the `std::assets` module scope.
    ///
    fn module_assets() -> Rc<RefCell<Scope>> {
        let std_assets_scope = Scope::new_built_in("assets").wrap();

        let std_assets_token_scope = Scope::new_built_in("Token").wrap();
        let std_assets_token_transfer =
            FunctionType::new_std(FunctionIdentifier::AssetsTokenTransfer);
        Scope::insert_item(
            std_assets_token_scope.clone(),
            std_assets_token_transfer.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(std_assets_token_transfer),
                true,
            ))
            .wrap(),
        );
        let std_assets_token = StructureType::new(
            None,
            std_assets_token_scope.borrow().name(),
            BuiltInTypeId::StdAssetsToken as usize,
            vec![],
            Some(std_assets_token_scope.clone()),
        );
        Scope::insert_item(
            std_assets_scope.clone(),
            std_assets_token_scope.borrow().name(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Structure(std_assets_token),
                false,
            ))
            .wrap(),
        );

        std_assets_scope
    }
}
