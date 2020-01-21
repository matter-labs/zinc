extern crate franklin_crypto;

use crate::gadgets::PrimitiveOperations;
use crate::vm::{InternalVM, RuntimeError, VMInstruction, VirtualMachine};
use crate::ZincEngine;
use num_bigint::ToBigInt;
use num_traits::Signed;
use zinc_bytecode::instructions::Dbg;

impl<E, O> VMInstruction<E, O> for Dbg
where
    E: ZincEngine,
    O: PrimitiveOperations<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let mut args = Vec::new();
        for _ in 0..self.nargs {
            args.push(vm.pop()?.value()?);
        }

        if let Some(condition) = vm.condition_top()?.to_bigint() {
            if condition.is_positive() {
                print!("{}", self.string);
                for value in args.iter() {
                    print!(" {}", value)
                }
                println!();
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
            .add(PushConst::new_untyped(42.into()))
            .add(Dbg::new("Value: ".into(), 1))
            .test::<u32>(&[])
            .unwrap();
    }
}
