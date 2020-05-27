use crate::core::{VMInstruction, VirtualMachine};

use crate::{gadgets, Result};

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::instructions::BitShiftRight;

impl<VM: VirtualMachine> VMInstruction<VM> for BitShiftRight {
    fn execute(&self, vm: &mut VM) -> Result {
        let shift = vm.pop()?.value()?;
        let num = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::shift_right(cs.namespace(|| "shift right"), &num, &shift)?;
        vm.push(result.into())
    }
}
