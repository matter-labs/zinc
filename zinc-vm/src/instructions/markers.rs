//!
//! The `*Marker` instructions.
//!

use zinc_types::ColumnMarker;
use zinc_types::FileMarker;
use zinc_types::FunctionMarker;
use zinc_types::LineMarker;

use crate::core::location::Location;
use crate::core::virtual_machine::IVirtualMachine;
use crate::error::Error;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for FileMarker {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        vm.set_location(Location {
            file: Some(self.file),
            function: None,
            line: None,
            column: None,
        });

        Ok(())
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for FunctionMarker {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut location = vm.get_location();
        location.function = Some(self.function);
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for LineMarker {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut location = vm.get_location();
        location.line = Some(self.line);
        vm.set_location(location);
        Ok(())
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for ColumnMarker {
    fn execute(self, vm: &mut VM) -> Result<(), Error> {
        let mut location = vm.get_location();
        location.column = Some(self.column);
        vm.set_location(location);
        Ok(())
    }
}
