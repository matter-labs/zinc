//!
//! The semantic analyzer scope intrinsic items.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_build::LibraryFunctionIdentifier;

use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::module::Module as ScopeModuleItem;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::Item as ScopeItem;
use crate::semantic::scope::Scope;

///
/// An intrinsic items set instance creator.
///
/// The intrinsic items are functions `dbg!` and `require` and the `std` and `zksync` libraries.
///
#[derive(Debug)]
pub struct IntrinsicScope {}

///
/// The intrinsic structures type IDs.
///
pub enum IntrinsicTypeId {
    /// The `std::crypto::ecc::Point` structure type ID.
    StdCryptoEccPoint = 0,
    /// The `std::crypto::schnorr::Signature` structure type ID.
    StdCryptoSchnorrSignature = 1,
}

impl IntrinsicScope {
    ///
    /// Initializes the intrinsic module scope.
    ///
    pub fn initialize() -> Rc<RefCell<Scope>> {
        let intrinsic_scope = Scope::new_intrinsic("intrinsic").wrap();

        let intrinsic_function_dbg = FunctionType::new_dbg();
        Scope::insert_item(
            intrinsic_scope.clone(),
            intrinsic_function_dbg.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(intrinsic_function_dbg),
                false,
            ))
            .wrap(),
        );

        let intrinsic_function_assert = FunctionType::new_require();
        Scope::insert_item(
            intrinsic_scope.clone(),
            intrinsic_function_assert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(intrinsic_function_assert),
                false,
            ))
            .wrap(),
        );

        Scope::insert_item(
            intrinsic_scope.clone(),
            "std".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "std".to_owned(),
                Self::module_std(),
            ))
            .wrap(),
        );

        Scope::insert_item(
            intrinsic_scope.clone(),
            "zksync".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "zksync".to_owned(),
                Self::module_zksync(),
            ))
            .wrap(),
        );

        intrinsic_scope
    }

    ///
    /// Initializes the `std` module scope.
    ///
    fn module_std() -> Rc<RefCell<Scope>> {
        let std_scope = Scope::new_intrinsic("std").wrap();

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

        std_scope
    }

    ///
    /// Initializes the `std::crypto` module scope.
    ///
    fn module_crypto() -> Rc<RefCell<Scope>> {
        let std_crypto_scope = Scope::new_intrinsic("crypto").wrap();

        let std_crypto_sha256 = FunctionType::new_library(LibraryFunctionIdentifier::CryptoSha256);
        let std_crypto_pedersen =
            FunctionType::new_library(LibraryFunctionIdentifier::CryptoPedersen);

        let std_crypto_schnorr_scope = Scope::new_intrinsic("schnorr").wrap();
        let std_crypto_schnorr_signature_scope = Scope::new_intrinsic("Signature").wrap();
        let std_crypto_schnorr_verify =
            FunctionType::new_library(LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify);
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
            IntrinsicTypeId::StdCryptoEccPoint as usize,
            vec![
                ("x".to_owned(), Type::field(None)),
                ("y".to_owned(), Type::field(None)),
            ],
            None,
        );
        let std_crypto_schnorr_signature = StructureType::new(
            None,
            std_crypto_schnorr_signature_scope.borrow().name(),
            IntrinsicTypeId::StdCryptoSchnorrSignature as usize,
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

        let std_crypto_ecc_scope = Scope::new_intrinsic("ecc").wrap();
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
        let std_convert_scope = Scope::new_intrinsic("convert").wrap();

        let std_convert_to_bits =
            FunctionType::new_library(LibraryFunctionIdentifier::ConvertToBits);
        let std_convert_from_bits_unsigned =
            FunctionType::new_library(LibraryFunctionIdentifier::ConvertFromBitsUnsigned);
        let std_convert_from_bits_signed =
            FunctionType::new_library(LibraryFunctionIdentifier::ConvertFromBitsSigned);
        let std_convert_from_bits_field =
            FunctionType::new_library(LibraryFunctionIdentifier::ConvertFromBitsField);

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
        let std_array_scope = Scope::new_intrinsic("array").wrap();

        let std_array_reverse = FunctionType::new_library(LibraryFunctionIdentifier::ArrayReverse);
        let std_array_truncate =
            FunctionType::new_library(LibraryFunctionIdentifier::ArrayTruncate);
        let std_array_pad = FunctionType::new_library(LibraryFunctionIdentifier::ArrayPad);

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
        let std_ff_scope = Scope::new_intrinsic("ff").wrap();

        let std_ff_invert = FunctionType::new_library(LibraryFunctionIdentifier::FfInvert);

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
    /// Initializes the `zksync` module scope.
    ///
    fn module_zksync() -> Rc<RefCell<Scope>> {
        let zksync_scope = Scope::new_intrinsic("zksync").wrap();

        let zksync_transfer = FunctionType::new_library(LibraryFunctionIdentifier::ZksyncTransfer);

        Scope::insert_item(
            zksync_scope.clone(),
            zksync_transfer.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(
                Type::Function(zksync_transfer),
                false,
            ))
            .wrap(),
        );

        zksync_scope
    }
}
