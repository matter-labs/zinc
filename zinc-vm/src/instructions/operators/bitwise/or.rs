use franklin_crypto::bellman::ConstraintSystem;

use zinc_bytecode::BitwiseOr;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;
use crate::gadgets::bitwise;
impl<VM: VirtualMachine> VMInstruction<VM> for BitwiseOr {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let right = vm.pop()?.value()?;
        let left = vm.pop()?.value()?;
        let cs = vm.constraint_system();
        let result = bitwise::or::bit_or(cs.namespace(|| "bit_and"), &left, &right)?;
        vm.push(result.into())
    }
}