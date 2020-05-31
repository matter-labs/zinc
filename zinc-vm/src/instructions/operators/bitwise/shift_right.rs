use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseShiftRight;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
impl<VM: VirtualMachine> VMInstruction<VM> for BitwiseShiftRight {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let shift = vm.pop()?.try_into_value()?;
        let num = vm.pop()?.try_into_value()?;
        let cs = vm.constraint_system();
        let result =
            bitwise::shift_right::shift_right(cs.namespace(|| "shift right"), &num, &shift)?;
        vm.push(result.into())
    }
}
