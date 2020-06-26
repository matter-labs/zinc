//!
//! The function instructions.
//!

use zinc_bytecode::Call;
use zinc_bytecode::Return;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Call {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.call(self.address, self.input_size)
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for Return {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.r#return(self.output_size)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test() -> Result<(), TestingError> {
        TestRunner::new()
            // call main
            .push(zinc_bytecode::Call::new(11, 0))
            // fn min(field, field) -> field
            .push(zinc_bytecode::Load::new(0, 1))
            .push(zinc_bytecode::Load::new(1, 1))
            .push(zinc_bytecode::Lt)
            .push(zinc_bytecode::If)
            .push(zinc_bytecode::Load::new(0, 1))
            .push(zinc_bytecode::Else)
            .push(zinc_bytecode::Load::new(1, 1))
            .push(zinc_bytecode::EndIf)
            .push(zinc_bytecode::Return::new(1))
            // fn main
            .push(zinc_bytecode::Push::new_field(42.into()))
            .push(zinc_bytecode::Push::new_field(5.into()))
            .push(zinc_bytecode::Push::new_field(3.into()))
            .push(zinc_bytecode::Call::new(2, 2))
            .test(&[3, 42])
    }
}
