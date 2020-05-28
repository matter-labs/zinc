use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseShiftLeft;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
impl<VM: VirtualMachine> VMInstruction<VM> for BitwiseShiftLeft {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let shift = vm.pop()?.value()?;
        let num = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = bitwise::shift_left::shift_left(cs.namespace(|| "shift left"), &num, &shift)?;
        vm.push(result.into())
    }
}
