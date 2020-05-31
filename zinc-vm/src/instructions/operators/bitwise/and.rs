use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseAnd;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
impl<VM: VirtualMachine> VMInstruction<VM> for BitwiseAnd {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.try_into_value()?;
        let left = vm.pop()?.try_into_value()?;
        let cs = vm.constraint_system();
        let result = bitwise::and::bit_and(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}
