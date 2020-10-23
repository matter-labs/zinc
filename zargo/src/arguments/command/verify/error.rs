//!
//! The Zargo package manager `verify` subcommand.
//!

use failure::Fail;

use crate::executable::virtual_machine::Error as VirtualMachineError;

///
/// The Zargo package manager `verify` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest {}", _0)]
    Manifest(zinc_manifest::Error),
    /// The contract method to call is missing.
    #[fail(display = "contract method to call must be specified")]
    MethodMissing,
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
}
