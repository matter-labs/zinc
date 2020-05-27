
use crate::core::{VirtualMachine, VMInstruction};
use crate::core::{RuntimeError};
use crate::stdlib::crypto::VerifySchnorrSignature;
use crate::{stdlib};
use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::instructions::CallBuiltin;

impl<VM: VirtualMachine> VMInstruction<VM> for CallBuiltin {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        match self.identifier {
            BuiltinIdentifier::CryptoSchnorrSignatureVerify => {
                vm.call_native(VerifySchnorrSignature::new(self.inputs_count)?)
            }
            BuiltinIdentifier::FieldInverse => vm.call_native(stdlib::ff::Inverse),
            BuiltinIdentifier::CryptoSha256 => {
                vm.call_native(stdlib::crypto::Sha256::new(self.inputs_count)?)
            }
            BuiltinIdentifier::CryptoPedersen => {
                vm.call_native(stdlib::crypto::Pedersen::new(self.inputs_count)?)
            }
            BuiltinIdentifier::ToBits => vm.call_native(stdlib::bits::ToBits),
            BuiltinIdentifier::UnsignedFromBits => {
                vm.call_native(stdlib::bits::UnsignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::SignedFromBits => {
                vm.call_native(stdlib::bits::SignedFromBits::new(self.inputs_count))
            }
            BuiltinIdentifier::FieldFromBits => vm.call_native(stdlib::bits::FieldFromBits),
            BuiltinIdentifier::ArrayReverse => {
                vm.call_native(stdlib::array::Reverse::new(self.inputs_count)?)
            }
            BuiltinIdentifier::ArrayTruncate => {
                vm.call_native(stdlib::array::Truncate::new(self.inputs_count)?)
            }
            BuiltinIdentifier::ArrayPad => {
                vm.call_native(stdlib::array::Pad::new(self.inputs_count)?)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_cast() -> Result<(), TestingError> {
        Ok(())
    }
}
