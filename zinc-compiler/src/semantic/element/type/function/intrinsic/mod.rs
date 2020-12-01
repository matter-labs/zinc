//!
//! The semantic analyzer intrinsic function element.
//!

#[cfg(test)]
mod tests;

pub mod contract_fetch;
pub mod contract_transfer;
pub mod debug;
pub mod require;
pub mod stdlib;

use std::fmt;

use zinc_lexical::Location;
use zinc_types::LibraryFunctionIdentifier;

use crate::semantic::element::r#type::contract::Contract as ContractType;

use self::contract_fetch::Function as ContractFetchFunction;
use self::contract_transfer::Function as ContractTransferFunction;
use self::debug::Function as DebugFunction;
use self::require::Function as RequireFunction;
use self::stdlib::array_pad::Function as StdArrayPadFunction;
use self::stdlib::array_reverse::Function as StdArrayReverseFunction;
use self::stdlib::array_truncate::Function as StdArrayTruncateFunction;
use self::stdlib::collections_mtreemap_contains::Function as StdCollectionsMTreeMapContainsFunction;
use self::stdlib::collections_mtreemap_get::Function as StdCollectionsMTreeMapGetFunction;
use self::stdlib::collections_mtreemap_insert::Function as StdCollectionsMTreeMapInsertFunction;
use self::stdlib::collections_mtreemap_remove::Function as StdCollectionsMTreeMapRemoveFunction;
use self::stdlib::convert_from_bits_field::Function as StdConvertFromBitsFieldFunction;
use self::stdlib::convert_from_bits_signed::Function as StdConvertFromBitsSignedFunction;
use self::stdlib::convert_from_bits_unsigned::Function as StdConvertFromBitsUnsignedFunction;
use self::stdlib::convert_to_bits::Function as StdConvertToBitsFunction;
use self::stdlib::crypto_pedersen::Function as StdConvertPedersenFunction;
use self::stdlib::crypto_schnorr_signature_verify::Function as StdCryptoSchnorrSignatureVerifyFunction;
use self::stdlib::crypto_sha256::Function as StdCryptoSha256Function;
use self::stdlib::ff_invert::Function as StdFfInvertFunction;
use self::stdlib::Function as StandardLibraryFunction;

///
/// The semantic analyzer intrinsic function element.
///
#[derive(Debug, Clone)]
pub enum Function {
    /// The `require(...)` function. See the inner element description.
    Require(RequireFunction),
    /// The `dbg!(...)` function. See the inner element description.
    Debug(DebugFunction),
    /// The `<Contract>::fetch(...)` function. See the inner element description.
    ContractFetch(ContractFetchFunction),
    /// The `<Contract>::transfer(...)` function. See the inner element description.
    ContractTransfer(ContractTransferFunction),
    /// The standard library function. See the inner element description.
    StandardLibrary(StandardLibraryFunction),
}

impl Function {
    ///
    /// A shortcut constructor.
    ///
    pub fn require() -> Self {
        Self::Require(RequireFunction::default())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn debug() -> Self {
        Self::Debug(DebugFunction::default())
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn contract_fetch(contract_type: ContractType) -> Self {
        Self::ContractFetch(ContractFetchFunction::new(contract_type))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn contract_transfer(contract_type: ContractType) -> Self {
        Self::ContractFetch(ContractFetchFunction::new(contract_type))
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn library(identifier: LibraryFunctionIdentifier) -> Self {
        match identifier {
            LibraryFunctionIdentifier::CryptoSha256 => Self::StandardLibrary(
                StandardLibraryFunction::CryptoSha256(StdCryptoSha256Function::default()),
            ),
            LibraryFunctionIdentifier::CryptoPedersen => Self::StandardLibrary(
                StandardLibraryFunction::CryptoPedersen(StdConvertPedersenFunction::default()),
            ),
            LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify => {
                Self::StandardLibrary(StandardLibraryFunction::CryptoSchnorrSignatureVerify(
                    StdCryptoSchnorrSignatureVerifyFunction::default(),
                ))
            }

            LibraryFunctionIdentifier::ConvertToBits => Self::StandardLibrary(
                StandardLibraryFunction::ConvertToBits(StdConvertToBitsFunction::default()),
            ),
            LibraryFunctionIdentifier::ConvertFromBitsUnsigned => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsUnsigned(
                    StdConvertFromBitsUnsignedFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::ConvertFromBitsSigned => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsSigned(
                    StdConvertFromBitsSignedFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::ConvertFromBitsField => {
                Self::StandardLibrary(StandardLibraryFunction::ConvertFromBitsField(
                    StdConvertFromBitsFieldFunction::default(),
                ))
            }

            LibraryFunctionIdentifier::ArrayReverse => Self::StandardLibrary(
                StandardLibraryFunction::ArrayReverse(StdArrayReverseFunction::default()),
            ),
            LibraryFunctionIdentifier::ArrayTruncate => Self::StandardLibrary(
                StandardLibraryFunction::ArrayTruncate(StdArrayTruncateFunction::default()),
            ),
            LibraryFunctionIdentifier::ArrayPad => Self::StandardLibrary(
                StandardLibraryFunction::ArrayPad(StdArrayPadFunction::default()),
            ),

            LibraryFunctionIdentifier::FfInvert => Self::StandardLibrary(
                StandardLibraryFunction::FfInvert(StdFfInvertFunction::default()),
            ),

            LibraryFunctionIdentifier::ContractTransfer => {
                Self::ContractTransfer(ContractTransferFunction::default())
            }

            LibraryFunctionIdentifier::CollectionsMTreeMapGet => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapGet(
                    StdCollectionsMTreeMapGetFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapContains => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapContains(
                    StdCollectionsMTreeMapContainsFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapInsert => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapInsert(
                    StdCollectionsMTreeMapInsertFunction::default(),
                ))
            }
            LibraryFunctionIdentifier::CollectionsMTreeMapRemove => {
                Self::StandardLibrary(StandardLibraryFunction::CollectionsMTreeMapRemove(
                    StdCollectionsMTreeMapRemoveFunction::default(),
                ))
            }
        }
    }

    ///
    /// Whether the function requires the Rust-macro-like `!` specifier.
    ///
    pub fn requires_exclamation_mark(&self) -> bool {
        matches!(self, Self::Debug(_))
    }

    ///
    /// Whether the function must be called from mutable context.
    ///
    pub fn is_mutable(&self) -> bool {
        match self {
            Self::Require(_) => false,
            Self::Debug(_) => false,
            Self::ContractFetch(_) => false,
            Self::ContractTransfer(_) => true,
            Self::StandardLibrary(inner) => inner.is_mutable(),
        }
    }

    ///
    /// Returns the function identifier, which is known at compile time.
    ///
    pub fn identifier(&self) -> &'static str {
        match self {
            Self::Require(inner) => inner.identifier,
            Self::Debug(inner) => inner.identifier,
            Self::ContractFetch(inner) => inner.identifier,
            Self::ContractTransfer(inner) => inner.identifier,
            Self::StandardLibrary(inner) => inner.identifier(),
        }
    }

    ///
    /// Sets the function call location in the code.
    ///
    pub fn set_location(&mut self, location: Location) {
        match self {
            Self::Require(inner) => inner.location = Some(location),
            Self::Debug(inner) => inner.location = Some(location),
            Self::ContractFetch(inner) => inner.location = Some(location),
            Self::ContractTransfer(inner) => inner.location = Some(location),
            Self::StandardLibrary(inner) => inner.set_location(location),
        }
    }

    ///
    /// Returns the location of the function call.
    ///
    pub fn location(&self) -> Option<Location> {
        match self {
            Self::Require(inner) => inner.location,
            Self::Debug(inner) => inner.location,
            Self::ContractFetch(inner) => inner.location,
            Self::ContractTransfer(inner) => inner.location,
            Self::StandardLibrary(inner) => inner.location(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Require(inner) => write!(f, "{}", inner),
            Self::Debug(inner) => write!(f, "{}", inner),
            Self::ContractFetch(inner) => write!(f, "{}", inner),
            Self::ContractTransfer(inner) => write!(f, "{}", inner),
            Self::StandardLibrary(inner) => write!(f, "std::{}", inner),
        }
    }
}
