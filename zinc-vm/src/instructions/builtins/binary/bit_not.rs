use crate::core::{VirtualMachine, VMInstruction};

use crate::{gadgets, Result};

use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::instructions::BitNot;

impl<VM: VirtualMachine> VMInstruction<VM> for BitNot {
    fn execute(&self, vm: &mut VM) -> Result {
        let scalar = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_not(cs.namespace(|| "bit_and"), &scalar)?;
        vm.push(result.into())
    }
}
