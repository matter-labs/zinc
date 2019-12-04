extern crate franklin_crypto;

use crate::element::{Element, ElementOperator};
use crate::vm::{VMInstruction, InternalVM};
use crate::vm::{RuntimeError, VirtualMachine};
use zinc_bytecode::instructions::Neg;

impl<E, O> VMInstruction<E, O> for Neg
where
    E: Element,
    O: ElementOperator<E>,
{
    fn execute(&self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let element = vm.pop()?;
        let neg = vm.get_operator().neg(element)?;

        vm.push(neg)
    }
}

#[cfg(test)]
mod test {
    use crate::instructions::testing_utils::TestingError;

    #[test]
    fn test_neg() -> Result<(), TestingError> {
        Ok(())
    }
}
