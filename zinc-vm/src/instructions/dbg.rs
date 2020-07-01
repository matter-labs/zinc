//!
//! The `Debug` instruction.
//!

use num_bigint::ToBigInt;
use num_traits::Signed;

use franklin_crypto::bellman::SynthesisError;

use zinc_bytecode::Dbg;
use zinc_bytecode::TemplateValue;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Dbg {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut values = Vec::with_capacity(self.argument_types.len());

        for argument_type in self.argument_types.into_iter().rev() {
            let size = TemplateValue::new(argument_type.clone())
                .into_flat_values()
                .len();

            if vm.is_debugging() {
                let mut flat = Vec::with_capacity(size);
                for _ in 0..size {
                    let value = vm.pop()?.try_into_value()?.to_bigint().ok_or_else(|| {
                        RuntimeError::SynthesisError(SynthesisError::AssignmentMissing)
                    })?;
                    flat.push(value);
                }
                flat.reverse();
                let value = TemplateValue::new_from_flat_values(argument_type, &flat)
                    .expect(zinc_const::panic::VALUE_ALWAYS_EXISTS);
                values.push(value);
            };
        }

        if let Some(condition) = vm.condition_top()?.to_bigint() {
            if condition.is_positive() && vm.is_debugging() {
                let mut buffer = self.format;
                for value in values.into_iter().rev() {
                    let json = serde_json::to_string(&value.try_into_json())
                        .expect(zinc_const::panic::DATA_SERIALIZATION);
                    buffer = buffer.replacen("{}", &json, 1);
                }
                eprintln!("{}", buffer);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestRunner;

    #[test]
    fn test() {
        TestRunner::new()
            .push(zinc_bytecode::Push::new_field(42.into()))
            .push(zinc_bytecode::Dbg::new("Value: {}".into(), vec![]))
            .test::<u32>(&[])
            .expect(zinc_const::panic::TEST_DATA_VALID);
    }
}
