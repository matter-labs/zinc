//!
//! The Zargo package manager `proof-check` subcommand.
//!

use failure::Fail;

use crate::error::directory::Error as DirectoryError;
use crate::error::file::Error as FileError;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;

///
/// The Zargo package manager `proof-check` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest {}", _0)]
    Manifest(zinc_manifest::Error),
    /// The contract method to call is missing.
    #[fail(display = "contract method to call must be specified")]
    MethodMissing,
    /// The project binary build directory error.
    #[fail(display = "build directory {}", _0)]
    BuildDirectory(DirectoryError),
    /// The project template, keys, and other auxiliary data directory error.
    #[fail(display = "data directory {}", _0)]
    DataDirectory(DirectoryError),
    /// The private key file generation error.
    #[fail(display = "private key file {}", _0)]
    PrivateKeyFile(FileError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The virtual machine `run` process error.
    #[fail(display = "virtual machine 'run' {}", _0)]
    VirtualMachineRun(VirtualMachineError),
    /// The virtual machine `setup` process error.
    #[fail(display = "virtual machine 'setup' {}", _0)]
    VirtualMachineSetup(VirtualMachineError),
    /// The virtual machine `proof-check` process error.
    #[fail(display = "virtual machine 'prove & verify' {}", _0)]
    VirtualMachineProveAndVerify(VirtualMachineError),
}
