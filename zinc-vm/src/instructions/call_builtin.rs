extern crate franklin_crypto;

use crate::gadgets::stdlib::crypto::{Pedersen, Sha256};
use crate::gadgets::stdlib::bits::*;
use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::instructions::CallBuiltin;
use crate::gadgets::stdlib::arrays::{ArrayPad, Truncate, Reverse};

impl<E, O> VMInstruction<E, O> for CallBuiltin
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let mut input = Vec::new();
        for _ in 0..self.inputs_count {
            let value = vm.pop()?.value()?;
            input.push(value);
        }

        let output = match self.identifier {
            BuiltinIdentifier::CryptoSha256 => vm.operations().execute(Sha256, &input),
            BuiltinIdentifier::CryptoPedersen => vm.operations().execute(Pedersen, &input),
            BuiltinIdentifier::ToBits => vm.operations().execute(ToBits, &input),
            BuiltinIdentifier::UnsignedFromBits => vm.operations().execute(UnsignedFromBits, &input),
            BuiltinIdentifier::SignedFromBits => vm.operations().execute(SignedFromBits, &input),
            BuiltinIdentifier::ArrayPad => vm.operations().execute(ArrayPad, &input),
            BuiltinIdentifier::ArrayTruncate => vm.operations().execute(Truncate, &input),
            BuiltinIdentifier::ArrayReverse => vm.operations().execute(Reverse, &input),
            f => unimplemented!("Builtin function {} is not implemented.", f)
        }?;

        for value in output {
            vm.push(Cell::Value(value))?
        }

        Ok(())
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
