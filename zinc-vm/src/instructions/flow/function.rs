//!
//! The function instructions.
//!

use zinc_build::Call;
use zinc_build::Return;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for Call {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        vm.call(self.address, self.input_size)
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for Return {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        vm.r#return(self.output_size)
    }
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    fn test() -> Result<(), TestingError> {
        TestRunner::new()
            // call main
            .push(zinc_build::Call::new(11, 0))
            // fn min(field, field) -> field
            .push(zinc_build::Load::new(0, 1))
            .push(zinc_build::Load::new(1, 1))
            .push(zinc_build::Lt)
            .push(zinc_build::If)
            .push(zinc_build::Load::new(0, 1))
            .push(zinc_build::Else)
            .push(zinc_build::Load::new(1, 1))
            .push(zinc_build::EndIf)
            .push(zinc_build::Return::new(1))
            // fn main
            .push(zinc_build::Push::new_field(BigInt::from(42)))
            .push(zinc_build::Push::new_field(BigInt::from(5)))
            .push(zinc_build::Push::new_field(BigInt::from(3)))
            .push(zinc_build::Call::new(2, 2))
            .test(&[3, 42])
    }
}
