use crate::core::{VMInstruction, VirtualMachine};
use crate::{Result};

use crate::gadgets;
use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitAnd;

impl<VM: VirtualMachine> VMInstruction<VM> for BitAnd {
    fn execute(&self, vm: &mut VM) -> Result {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_and(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}
