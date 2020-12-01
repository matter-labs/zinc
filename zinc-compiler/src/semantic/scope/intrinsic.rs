//!
//! The semantic analyzer scope intrinsic items.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::r#type::function::Function as FunctionType;
use crate::semantic::element::r#type::structure::Structure as StructureType;
use crate::semantic::element::r#type::Type;
use crate::semantic::scope::item::module::Module as ScopeModuleItem;
use crate::semantic::scope::item::r#type::Type as ScopeTypeItem;
use crate::semantic::scope::item::variable::Variable as ScopeVariableItem;
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
    /// The `zksync::Transaction` structure type ID.
    ZkSyncTransaction = 2,
    /// The `std::collections::MTreeMap` structure type ID.
    StdCollectionsMTreeMap = 3,
}

impl IntrinsicScope {
    ///
    /// Initializes the intrinsic module scope.
    ///
    pub fn initialize() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("intrinsic").wrap();

        let function_dbg = FunctionType::dbg();
        Scope::insert_item(
            scope.clone(),
            function_dbg.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(function_dbg))).wrap(),
        );

        let function_require = FunctionType::require();
        Scope::insert_item(
            scope.clone(),
            function_require.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                function_require,
            )))
            .wrap(),
        );

        Scope::insert_item(
            scope.clone(),
            "std".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "std".to_owned(),
                Self::module_std(),
            ))
            .wrap(),
        );

        Scope::insert_item(
            scope.clone(),
            "zksync".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "zksync".to_owned(),
                Self::module_zksync(),
            ))
            .wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std` module scope.
    ///
    fn module_std() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("std").wrap();

        Scope::insert_item(
            scope.clone(),
            "crypto".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "crypto".to_owned(),
                Self::module_crypto(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            "convert".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "convert".to_owned(),
                Self::module_convert(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            "array".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "array".to_owned(),
                Self::module_array(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            "ff".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "ff".to_owned(),
                Self::module_ff(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            "collections".to_owned(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                "collections".to_owned(),
                Self::module_collections(),
            ))
            .wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std::crypto` module scope.
    ///
    fn module_crypto() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("crypto").wrap();

        let sha256 = FunctionType::library(LibraryFunctionIdentifier::CryptoSha256);
        let pedersen = FunctionType::library(LibraryFunctionIdentifier::CryptoPedersen);

        let schnorr_scope = Scope::new_intrinsic("schnorr").wrap();
        let schnorr_signature_scope = Scope::new_intrinsic("Signature").wrap();
        let schnorr_verify =
            FunctionType::library(LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify);
        Scope::insert_item(
            schnorr_signature_scope.clone(),
            schnorr_verify.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(schnorr_verify))).wrap(),
        );
        let ecc_point = StructureType::new(
            None,
            "Point".to_owned(),
            IntrinsicTypeId::StdCryptoEccPoint as usize,
            vec![
                ("x".to_owned(), Type::field(None)),
                ("y".to_owned(), Type::field(None)),
            ],
            None,
            None,
            schnorr_scope.clone(),
        );
        let schnorr_signature = StructureType::new(
            None,
            schnorr_signature_scope.borrow().name(),
            IntrinsicTypeId::StdCryptoSchnorrSignature as usize,
            vec![
                ("r".to_owned(), Type::Structure(ecc_point.clone())),
                ("s".to_owned(), Type::field(None)),
                ("pk".to_owned(), Type::Structure(ecc_point.clone())),
            ],
            None,
            None,
            schnorr_signature_scope.clone(),
        );
        Scope::insert_item(
            schnorr_scope.clone(),
            schnorr_signature_scope.borrow().name(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(
                schnorr_signature,
            )))
            .wrap(),
        );

        let ecc_scope = Scope::new_intrinsic("ecc").wrap();
        Scope::insert_item(
            ecc_scope.clone(),
            ecc_point.identifier.clone(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(ecc_point))).wrap(),
        );

        Scope::insert_item(
            scope.clone(),
            sha256.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(sha256))).wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            pedersen.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(pedersen))).wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            ecc_scope.borrow().name(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                ecc_scope.borrow().name(),
                ecc_scope.clone(),
            ))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            schnorr_scope.borrow().name(),
            ScopeItem::Module(ScopeModuleItem::new_built_in(
                schnorr_scope.borrow().name(),
                schnorr_scope.clone(),
            ))
            .wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std::convert` module scope.
    ///
    fn module_convert() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("convert").wrap();

        let to_bits = FunctionType::library(LibraryFunctionIdentifier::ConvertToBits);
        let from_bits_unsigned =
            FunctionType::library(LibraryFunctionIdentifier::ConvertFromBitsUnsigned);
        let from_bits_signed =
            FunctionType::library(LibraryFunctionIdentifier::ConvertFromBitsSigned);
        let from_bits_field =
            FunctionType::library(LibraryFunctionIdentifier::ConvertFromBitsField);

        Scope::insert_item(
            scope.clone(),
            to_bits.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(to_bits))).wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            from_bits_unsigned.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                from_bits_unsigned,
            )))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            from_bits_signed.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                from_bits_signed,
            )))
            .wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            from_bits_field.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(from_bits_field))).wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std::array` module scope.
    ///
    fn module_array() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("array").wrap();

        let reverse = FunctionType::library(LibraryFunctionIdentifier::ArrayReverse);
        let truncate = FunctionType::library(LibraryFunctionIdentifier::ArrayTruncate);
        let pad = FunctionType::library(LibraryFunctionIdentifier::ArrayPad);

        Scope::insert_item(
            scope.clone(),
            reverse.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(reverse))).wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            truncate.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(truncate))).wrap(),
        );
        Scope::insert_item(
            scope.clone(),
            pad.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(pad))).wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std::ff` module scope.
    ///
    fn module_ff() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("ff").wrap();

        let invert = FunctionType::library(LibraryFunctionIdentifier::FfInvert);

        Scope::insert_item(
            scope.clone(),
            invert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(invert))).wrap(),
        );

        scope
    }

    ///
    /// Initializes the `std::collections` module scope.
    ///
    fn module_collections() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("collections").wrap();

        let merkle_tree_map_scope = Scope::new_intrinsic("MTreeMap").wrap();
        let merkle_tree_map = StructureType::new(
            None,
            "MTreeMap".to_owned(),
            IntrinsicTypeId::StdCollectionsMTreeMap as usize,
            vec![],
            Some(vec!["K".to_owned(), "V".to_owned()]),
            None,
            merkle_tree_map_scope.clone(),
        );
        let merkle_tree_map_get =
            FunctionType::library(LibraryFunctionIdentifier::CollectionsMTreeMapGet);
        Scope::insert_item(
            merkle_tree_map_scope.clone(),
            merkle_tree_map_get.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                merkle_tree_map_get,
            )))
            .wrap(),
        );
        let merkle_tree_map_contains =
            FunctionType::library(LibraryFunctionIdentifier::CollectionsMTreeMapContains);
        Scope::insert_item(
            merkle_tree_map_scope.clone(),
            merkle_tree_map_contains.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                merkle_tree_map_contains,
            )))
            .wrap(),
        );
        let merkle_tree_map_insert =
            FunctionType::library(LibraryFunctionIdentifier::CollectionsMTreeMapInsert);
        Scope::insert_item(
            merkle_tree_map_scope.clone(),
            merkle_tree_map_insert.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                merkle_tree_map_insert,
            )))
            .wrap(),
        );
        let merkle_tree_map_remove =
            FunctionType::library(LibraryFunctionIdentifier::CollectionsMTreeMapRemove);
        Scope::insert_item(
            merkle_tree_map_scope,
            merkle_tree_map_remove.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(
                merkle_tree_map_remove,
            )))
            .wrap(),
        );

        Scope::insert_item(
            scope.clone(),
            merkle_tree_map.identifier.clone(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(
                merkle_tree_map,
            )))
            .wrap(),
        );

        scope
    }

    ///
    /// Initializes the `zksync` module scope.
    ///
    fn module_zksync() -> Rc<RefCell<Scope>> {
        let scope = Scope::new_intrinsic("zksync").wrap();

        let transfer = FunctionType::library(LibraryFunctionIdentifier::ContractTransfer);

        Scope::insert_item(
            scope.clone(),
            transfer.identifier(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Function(transfer))).wrap(),
        );

        let transaction_type = StructureType::new(
            None,
            "Transaction".to_owned(),
            IntrinsicTypeId::ZkSyncTransaction as usize,
            vec![
                (
                    "sender".to_owned(),
                    Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
                ),
                (
                    "recipient".to_owned(),
                    Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
                ),
                (
                    "token_address".to_owned(),
                    Type::integer_unsigned(None, zinc_const::bitlength::ETH_ADDRESS),
                ),
                (
                    "amount".to_owned(),
                    Type::integer_unsigned(None, zinc_const::bitlength::BALANCE),
                ),
            ],
            None,
            None,
            scope.clone(),
        );
        Scope::insert_item(
            scope.clone(),
            transaction_type.identifier.clone(),
            ScopeItem::Type(ScopeTypeItem::new_built_in(Type::Structure(
                transaction_type.clone(),
            )))
            .wrap(),
        );

        Scope::insert_item(
            scope.clone(),
            zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned(),
            ScopeItem::Variable(ScopeVariableItem::new(
                None,
                false,
                zinc_const::contract::TRANSACTION_VARIABLE_NAME.to_owned(),
                Type::Structure(transaction_type),
            ))
            .wrap(),
        );

        scope
    }
}
