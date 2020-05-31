use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseNot;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
impl<VM: VirtualMachine> VMInstruction<VM> for BitwiseNot {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let scalar = vm.pop()?.try_into_value()?;
        let cs = vm.constraint_system();
        let result = bitwise::not::bit_not(cs.namespace(|| "bit_and"), &scalar)?;
        vm.push(result.into())
    }
}
