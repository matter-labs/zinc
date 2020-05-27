use crate::core::{VirtualMachine, VMInstruction};
use crate::{gadgets, Result};

use franklin_crypto::bellman::ConstraintSystem;
use zinc_bytecode::instructions::BitXor;

impl<VM: VirtualMachine> VMInstruction<VM> for BitXor {
    fn execute(&self, vm: &mut VM) -> Result {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = gadgets::bits::bit_xor(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}
