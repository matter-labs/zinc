use crate::core::{VirtualMachine, VMInstruction};

use crate::{gadgets, Result};

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::instructions::BitShiftLeft;

impl<VM: VirtualMachine> VMInstruction<VM> for BitShiftLeft {
    fn execute(&self, vm: &mut VM) -> Result {
        let shift = vm.pop()?.value()?;
        let num = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::shift_left(cs.namespace(|| "shift left"), &num, &shift)?;
        vm.push(result.into())
    }
}
