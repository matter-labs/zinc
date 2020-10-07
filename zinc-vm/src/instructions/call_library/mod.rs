//!
//! The standard library call instructions.
//!

pub mod array;
pub mod convert;
pub mod crypto;
pub mod ff;
pub mod zksync;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_build::CallLibrary;
use zinc_build::LibraryFunctionIdentifier;

use crate::core::execution_state::ExecutionState;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;
use crate::IEngine;

use self::array::pad::Pad as ArrayPad;
use self::array::reverse::Reverse as ArrayReverse;
use self::array::truncate::Truncate as ArrayTruncate;
use self::convert::from_bits_field::FromBitsField as ConvertFromBitsField;
use self::convert::from_bits_signed::FromBitsSigned as ConvertFromBitsSigned;
use self::convert::from_bits_unsigned::FromBitsUnsigned as ConvertFromBitsUnsigned;
use self::convert::to_bits::ToBits as ConvertToBits;
use self::crypto::pedersen::Pedersen as CryptoPedersen;
use self::crypto::schnorr_verify::SchnorrSignatureVerify as CryptoSchnorrSignatureVerify;
use self::crypto::sha256::Sha256 as CryptoSha256;
use self::ff::invert::Inverse as FfInverse;
use self::zksync::transfer::Transfer as ZksyncTransfer;

pub trait INativeCallable<E: IEngine> {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        state: &mut ExecutionState<E>,
    ) -> Result<(), RuntimeError>;
}

impl<VM: IVirtualMachine> IExecutable<VM> for CallLibrary {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        match self.identifier {
            LibraryFunctionIdentifier::CryptoSha256 => {
                vm.call_native(CryptoSha256::new(self.input_size)?)
            }
            LibraryFunctionIdentifier::CryptoPedersen => {
                vm.call_native(CryptoPedersen::new(self.input_size)?)
            }
            LibraryFunctionIdentifier::CryptoSchnorrSignatureVerify => {
                vm.call_native(CryptoSchnorrSignatureVerify::new(self.input_size)?)
            }

            LibraryFunctionIdentifier::ConvertToBits => vm.call_native(ConvertToBits),
            LibraryFunctionIdentifier::ConvertFromBitsUnsigned => {
                vm.call_native(ConvertFromBitsUnsigned::new(self.input_size))
            }
            LibraryFunctionIdentifier::ConvertFromBitsSigned => {
                vm.call_native(ConvertFromBitsSigned::new(self.input_size))
            }
            LibraryFunctionIdentifier::ConvertFromBitsField => vm.call_native(ConvertFromBitsField),

            LibraryFunctionIdentifier::ArrayReverse => {
                vm.call_native(ArrayReverse::new(self.input_size)?)
            }
            LibraryFunctionIdentifier::ArrayTruncate => {
                vm.call_native(ArrayTruncate::new(self.input_size)?)
            }
            LibraryFunctionIdentifier::ArrayPad => vm.call_native(ArrayPad::new(self.input_size)?),

            LibraryFunctionIdentifier::FfInvert => vm.call_native(FfInverse),

            LibraryFunctionIdentifier::ZksyncTransfer => vm.call_native(ZksyncTransfer),
        }
    }
}
