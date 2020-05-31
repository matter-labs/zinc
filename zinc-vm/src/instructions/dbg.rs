use num_bigint::ToBigInt;
use num_traits::Signed;

use franklin_crypto::bellman::SynthesisError;

use zinc_bytecode::Dbg;
use zinc_bytecode::TemplateValue;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Dbg {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut values = Vec::with_capacity(self.arg_types.len());

        for arg_type in self.arg_types.iter().rev() {
            let size = TemplateValue::default_from_type(arg_type)
                .to_flat_values()
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
                let value =
                    TemplateValue::from_flat_values(arg_type, &flat).expect("value size is known");
                values.push(value);
            };
        }

        if let Some(condition) = vm.condition_top()?.to_bigint() {
            if condition.is_positive() && vm.is_debugging() {
                let mut buffer = self.format.clone();
                for value in values.into_iter().rev() {
                    let json = serde_json::to_string(&value.to_json()).expect("valid json");
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
    use crate::tests::VMTestRunner;

    #[test]
    fn test() {
        VMTestRunner::new()
            .add(zinc_bytecode::Push::new_field(42.into()))
            .add(zinc_bytecode::Dbg::new("Value: {}".into(), vec![]))
            .test::<u32>(&[])
            .unwrap();
    }
}
