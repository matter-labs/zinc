//!
//! The standard library call instructions.
//!

pub mod array;
pub mod convert;
pub mod crypto;
pub mod ff;

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BuiltinIdentifier;
use zinc_bytecode::CallStd;

use crate::core::execution_state::evaluation_stack::EvaluationStack;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;
use crate::IEngine;

use self::array::pad::Pad;
use self::array::reverse::Reverse;
use self::array::truncate::Truncate;
use self::convert::from_bits_field::FieldFromBits;
use self::convert::from_bits_signed::SignedFromBits;
use self::convert::from_bits_unsigned::UnsignedFromBits;
use self::convert::to_bits::ToBits;
use self::crypto::pedersen::Pedersen;
use self::crypto::schnorr::VerifySchnorrSignature;
use self::crypto::sha256::Sha256;
use self::ff::inverse::Inverse;

pub trait INativeCallable<E: IEngine> {
    fn call<CS: ConstraintSystem<E>>(
        &self,
        cs: CS,
        stack: &mut EvaluationStack<E>,
    ) -> Result<(), RuntimeError>;
}

impl<VM: IVirtualMachine> IExecutable<VM> for CallStd {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        match self.identifier {
            BuiltinIdentifier::CryptoSha256 => vm.call_native(Sha256::new(self.inputs_count)?),
            BuiltinIdentifier::CryptoPedersen => vm.call_native(Pedersen::new(self.inputs_count)?),
            BuiltinIdentifier::CryptoSchnorrSignatureVerify => {
                vm.call_native(VerifySchnorrSignature::new(self.inputs_count)?)
            }
            BuiltinIdentifier::UnsignedFromBits => {
                vm.call_native(UnsignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::SignedFromBits => {
                vm.call_native(SignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::FieldFromBits => vm.call_native(FieldFromBits),
            BuiltinIdentifier::ToBits => vm.call_native(ToBits),
            BuiltinIdentifier::ArrayReverse => vm.call_native(Reverse::new(self.inputs_count)?),
            BuiltinIdentifier::ArrayTruncate => vm.call_native(Truncate::new(self.inputs_count)?),
            BuiltinIdentifier::ArrayPad => vm.call_native(Pad::new(self.inputs_count)?),
            BuiltinIdentifier::FieldInverse => vm.call_native(Inverse),
        }
    }
}
