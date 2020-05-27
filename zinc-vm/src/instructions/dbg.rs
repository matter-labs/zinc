extern crate franklin_crypto;

use self::franklin_crypto::bellman::{ConstraintSystem, SynthesisError};
use crate::core::{InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use crate::Engine;
use num_bigint::ToBigInt;
use num_traits::Signed;
use zinc_bytecode::Dbg;
use zinc_bytecode::TemplateValue;

impl<E, CS> VMInstruction<E, CS> for Dbg
where
    E: Engine,
    CS: ConstraintSystem<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, CS>) -> Result<(), RuntimeError> {
        let mut values = Vec::with_capacity(self.arg_types.len());

        for arg_type in self.arg_types.iter().rev() {
            let size = TemplateValue::default_from_type(arg_type)
                .to_flat_values()
                .len();

            if vm.debugging {
                let mut flat = Vec::with_capacity(size);
                for _ in 0..size {
                    let value = vm.pop()?.value()?.to_bigint().ok_or_else(|| {
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
            if condition.is_positive() && vm.debugging {
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
    use super::*;
    use crate::instructions::testing_utils::VMTestRunner;
    use zinc_bytecode::PushConst;

    #[test]
    fn test() {
        VMTestRunner::new()
            .add(PushConst::new_field(42.into()))
            .add(Dbg::new("Value: {}".into(), vec![]))
            .test::<u32>(&[])
            .unwrap();
    }
}
