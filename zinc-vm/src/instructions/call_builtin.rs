<<<<<<< HEAD
use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BuiltinIdentifier;
use zinc_bytecode::CallStd;

use crate::core::InternalVM;
use crate::core::RuntimeError;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::stdlib::array::pad::Pad;
use crate::stdlib::array::reverse::Reverse;
use crate::stdlib::array::truncate::Truncate;
use crate::stdlib::bits::field_from_bits::FieldFromBits;
use crate::stdlib::bits::signed_from_bits::SignedFromBits;
use crate::stdlib::bits::to_bits::ToBits;
use crate::stdlib::bits::unsigned_from_bits::UnsignedFromBits;
use crate::stdlib::crypto::pedersen::Pedersen;
use crate::stdlib::crypto::schnorr::VerifySchnorrSignature;
use crate::stdlib::crypto::sha256::Sha256;
use crate::stdlib::ff::inverse::Inverse;
use crate::Engine;

impl<E, CS> VMInstruction<E, CS> for CallStd
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
=======
use crate::core::RuntimeError;
use crate::core::{VMInstruction, VirtualMachine};
use crate::stdlib;
use crate::stdlib::crypto::VerifySchnorrSignature;
use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::instructions::CallBuiltin;

impl<VM: VirtualMachine> VMInstruction<VM> for CallBuiltin {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
>>>>>>> am/storage
        match self.identifier {
            BuiltinIdentifier::CryptoSchnorrSignatureVerify => {
                vm.call_native(VerifySchnorrSignature::new(self.inputs_count)?)
            }
            BuiltinIdentifier::FieldInverse => vm.call_native(Inverse),
            BuiltinIdentifier::CryptoSha256 => vm.call_native(Sha256::new(self.inputs_count)?),
            BuiltinIdentifier::CryptoPedersen => vm.call_native(Pedersen::new(self.inputs_count)?),
            BuiltinIdentifier::ToBits => vm.call_native(ToBits),
            BuiltinIdentifier::UnsignedFromBits => {
                vm.call_native(UnsignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::SignedFromBits => {
                vm.call_native(SignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::FieldFromBits => vm.call_native(FieldFromBits),
            BuiltinIdentifier::ArrayReverse => vm.call_native(Reverse::new(self.inputs_count)?),
            BuiltinIdentifier::ArrayTruncate => vm.call_native(Truncate::new(self.inputs_count)?),
            BuiltinIdentifier::ArrayPad => vm.call_native(Pad::new(self.inputs_count)?),
        }
    }
}
