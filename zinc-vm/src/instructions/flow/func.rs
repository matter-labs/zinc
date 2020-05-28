use zinc_bytecode::Call;
use zinc_bytecode::Return;

use crate::core::VMInstruction;
use crate::core::VirtualMachine;
use crate::error::RuntimeError;

impl<VM: VirtualMachine> VMInstruction<VM> for Call {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.call(self.address, self.inputs_count)
    }
}

impl<VM: VirtualMachine> VMInstruction<VM> for Return {
    fn execute(&self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.ret(self.outputs_count)
    }
}

#[cfg(test)]
mod tests {

    //    #[test]
    //    fn test_func() -> Result<(), TestingError> {
    //        let _ = env_logger::builder().is_test(true).try_init();
    //
    //        VMTestRunner::new()
    //            // call main
    //            .add(Call::new(9, 0))
    //            // func min(field, field) -> field
    //            .add(Load::new(0))
    //            .add(Load::new(1))
    //            .add(Lt)
    //            .add(Load::new(0))
    //            .add(Load::new(1))
    //            .add(ConditionalSelect)
    //            .add(Return::new(1))
    //            // func main
    //            .add(Push::new_untyped(42.into()))
    //            .add(Push::new_untyped(5.into()))
    //            .add(Push::new_untyped(3.into()))
    //            .add(Call::new(2, 2))
    //            .test(&[3, 42])
    //    }
}
