extern crate franklin_crypto;

use crate::gadgets::stdlib::crypto::{Pedersen, Sha256};
use crate::gadgets::stdlib::from_bits::FromBits;
use crate::gadgets::stdlib::to_bits::ToBits;
use crate::gadgets::PrimitiveOperations;
use crate::vm::{Cell, InternalVM, VMInstruction};
use crate::vm::{RuntimeError, VirtualMachine};
use crate::ZincEngine;
use zinc_bytecode::builtins::BuiltinIdentifier;
use zinc_bytecode::instructions::CallBuiltin;

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
            BuiltinIdentifier::UnsignedFromBits => vm.operations().execute(FromBits, &input),
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
