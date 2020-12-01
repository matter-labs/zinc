//!
//! The `Debug` instruction.
//!

use num::bigint::ToBigInt;
use num::BigInt;
use num::Signed;

use franklin_crypto::bellman::SynthesisError;

use zinc_types::Dbg;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::gadgets::scalar::Scalar;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Dbg {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut values = Vec::with_capacity(self.argument_types.len());

        for argument_type in self.argument_types.into_iter().rev() {
            let size = argument_type.size();
            let mut flat = Vec::with_capacity(size);

            match argument_type {
                zinc_types::Type::Contract(fields) => {
                    let eth_address = vm.pop()?.try_into_value()?;

                    let mut flat = Vec::with_capacity(size);
                    for (index, field) in fields.iter().enumerate() {
                        let values: Vec<BigInt> = vm
                            .storage_load(
                                eth_address.clone(),
                                Scalar::new_constant_usize(
                                    index,
                                    zinc_types::ScalarType::Integer(zinc_types::IntegerType::new(
                                        false,
                                        zinc_const::bitlength::INDEX,
                                    )),
                                ),
                                field.r#type.size(),
                            )?
                            .into_iter()
                            .map(|scalar| scalar.to_bigint().unwrap_or_default())
                            .collect();
                        flat.extend(values);
                    }
                    values.push(zinc_types::Value::from_flat_values(
                        zinc_types::Type::Contract(fields),
                        flat.as_slice(),
                    ));
                }
                r#type => {
                    for _ in 0..size {
                        let value = vm
                            .pop()?
                            .try_into_value()?
                            .to_bigint()
                            .ok_or(Error::SynthesisError(SynthesisError::AssignmentMissing))?;
                        flat.push(value);
                    }
                    flat.reverse();
                    values.push(zinc_types::Value::from_flat_values(r#type, flat.as_slice()));
                }
            }
        }

        if let Some(condition) = vm.condition_top()?.to_bigint() {
            if condition.is_positive() {
                let mut buffer = self.format;
                for value in values.into_iter().rev() {
                    let json = serde_json::to_string(&value.into_json()).unwrap_or_default();
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
    use num::BigInt;

    use crate::tests::TestRunner;

    #[test]
    fn test() {
        TestRunner::new()
            .push(zinc_types::Push::new_field(BigInt::from(42)))
            .push(zinc_types::Dbg::new("Value: {}".into(), vec![]))
            .test::<u32>(&[])
            .expect(zinc_const::panic::TEST_DATA_VALID);
    }
}
