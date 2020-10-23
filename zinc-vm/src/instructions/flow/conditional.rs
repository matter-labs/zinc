//!
//! The conditional instructions.
//!

use zinc_build::Else;
use zinc_build::EndIf;
use zinc_build::If;

use crate::core::virtual_machine::IVirtualMachine;
use crate::error::RuntimeError;
use crate::instructions::IExecutable;

impl<VM: IVirtualMachine> IExecutable<VM> for If {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_then()
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for Else {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_else()
    }
}

impl<VM: IVirtualMachine> IExecutable<VM> for EndIf {
    fn execute(self, vm: &mut VM) -> Result<(), RuntimeError> {
        vm.branch_end()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp;

    use num::BigInt;
    use num::One;
    use num::Zero;

    use zinc_build::IntegerType;
    use zinc_build::ScalarType;

    use crate::tests::TestRunner;
    use crate::tests::TestingError;

    #[test]
    ///
    /// let a = _;
    /// let b = _;
    ///
    /// if a > b {
    ///     (a, b)
    /// } else {
    ///     (b, a)
    /// }
    ///
    fn test_evaluation_stack() -> Result<(), TestingError> {
        let data = [(5, 7), (7, 5), (6, 6)];

        for (a, b) in data.iter() {
            TestRunner::new()
                .push(zinc_build::Push::new((*a).into(), IntegerType::I8.into()))
                .push(zinc_build::Store::new(0, 1))
                .push(zinc_build::Push::new((*b).into(), IntegerType::I8.into()))
                .push(zinc_build::Store::new(1, 1))
                .push(zinc_build::Load::new(1, 1))
                .push(zinc_build::Load::new(0, 1))
                .push(zinc_build::Gt)
                .push(zinc_build::If)
                .push(zinc_build::Load::new(0, 1))
                .push(zinc_build::Load::new(1, 1))
                .push(zinc_build::Else)
                .push(zinc_build::Load::new(1, 1))
                .push(zinc_build::Load::new(0, 1))
                .push(zinc_build::EndIf)
                .test(&[cmp::max(*a, *b), cmp::min(*a, *b)])?;
        }

        Ok(())
    }

    #[test]
    ///
    /// let mut a = 0;
    /// let c = _;
    ///
    /// if c {
    ///     a += 1;
    /// } else {
    ///     a -= 1;
    /// }
    ///
    fn test_data_stack() -> Result<(), TestingError> {
        let data = [(1, 1), (0, -1)];

        for (c, r) in data.iter() {
            TestRunner::new()
                .push(zinc_build::Push::new(
                    BigInt::zero(),
                    IntegerType::I8.into(),
                ))
                .push(zinc_build::Store::new(0, 1))
                .push(zinc_build::Push::new((*c).into(), ScalarType::Boolean))
                .push(zinc_build::If)
                .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
                .push(zinc_build::Load::new(0, 1))
                .push(zinc_build::Add)
                .push(zinc_build::Store::new(0, 1))
                .push(zinc_build::Else)
                .push(zinc_build::Load::new(0, 1))
                .push(zinc_build::Push::new(BigInt::one(), IntegerType::I8.into()))
                .push(zinc_build::Sub)
                .push(zinc_build::Store::new(0, 1))
                .push(zinc_build::EndIf)
                .push(zinc_build::Load::new(0, 1))
                .test(&[*r])?;
        }

        Ok(())
    }
}
