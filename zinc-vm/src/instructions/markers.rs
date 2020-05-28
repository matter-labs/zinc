use zinc_bytecode::ColumnMarker;
use zinc_bytecode::FileMarker;
use zinc_bytecode::FunctionMarker;
use zinc_bytecode::LineMarker;

use crate::core::location::Location;
use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for FileMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.set_location(Location {
            file: Some(self.file.clone()),
            function: None,
            line: None,
            column: None,
        });

        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for FunctionMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.function = Some(self.function.clone());
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for LineMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.line = Some(self.line);
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for ColumnMarker {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        let mut location = vm.get_location();
        location.column = Some(self.column);
        vm.set_location(location);
        Ok(())
    }
}
