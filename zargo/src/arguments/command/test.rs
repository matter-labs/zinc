//!
//! The Zargo project manager `test` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use colored::Colorize;
use failure::Fail;
use structopt::StructOpt;

use zinc_const::UnitTestExitCode;

use crate::arguments::command::IExecutable;
use crate::directory::build::test::Directory as TestBuildDirectory;
use crate::directory::build::test::Error as TestBuildDirectoryError;
use crate::directory::build::Directory as BuildDirectory;
use crate::directory::data::Directory as DataDirectory;
use crate::directory::source::Directory as SourceDirectory;
use crate::executable::compiler::Compiler;
use crate::executable::compiler::Error as CompilerError;
use crate::executable::virtual_machine::Error as VirtualMachineError;
use crate::executable::virtual_machine::VirtualMachine;
use crate::manifest::Error as ManifestError;
use crate::manifest::Manifest;

///
/// The Zargo project manager `test` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project and saves its output")]
pub struct Command {
    /// The logging level value, which helps the logger to set the logging level.
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    pub verbosity: usize,

    /// The path to the Zargo project manifest file.
    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// The path to the binary bytecode test files directory.
    #[structopt(
        long = "binary",
        help = "Path to the bytecode test files directory",
        default_value = "./build/test"
    )]
    pub test_binaries_path: PathBuf,
}

///
/// The Zargo project manager `test` subcommand error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The manifest file error.
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    /// The compiler process error.
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    /// The project unit tests binary build directory error.
    #[fail(display = "test build directory {}", _0)]
    TestBuildDirectory(TestBuildDirectoryError),
    /// The virtual machine process error.
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
    /// The virtual machine process returned an unknown exit code.
    #[fail(display = "virtual machine unknown exit code")]
    UnknownExitCode(Option<i32>),
}

///
/// The unit test summary.
///
#[derive(Default)]
pub struct Summary {
    pub passed: u8,
    pub failed: u8,
    pub invalid: u8,
    pub ignored: u8,
}

impl IExecutable for Command {
    type Error = Error;

    fn execute(self) -> Result<(), Self::Error> {
        let _manifest = Manifest::try_from(&self.manifest_path).map_err(Error::ManifestFile)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        let source_directory_path = SourceDirectory::path(&manifest_path);
        let build_directory_path = BuildDirectory::path(&manifest_path);
        let data_directory_path = DataDirectory::path(&manifest_path);

        TestBuildDirectory::remove(&manifest_path).map_err(Error::TestBuildDirectory)?;
        TestBuildDirectory::create(&manifest_path).map_err(Error::TestBuildDirectory)?;

        Compiler::build_release(
            self.verbosity,
            &data_directory_path,
            &build_directory_path,
            &source_directory_path,
            true,
        )
        .map_err(Error::Compiler)?;

        let mut summary = Summary::default();

        for binary_path in TestBuildDirectory::files(&manifest_path)
            .map_err(Error::TestBuildDirectory)?
            .into_iter()
        {
            let status = VirtualMachine::test(self.verbosity, &binary_path)
                .map_err(Error::VirtualMachine)?;
            let code = UnitTestExitCode::try_from(status).map_err(Error::UnknownExitCode)?;

            match code {
                UnitTestExitCode::Passed => summary.passed += 1,
                UnitTestExitCode::Failed => summary.failed += 1,
                UnitTestExitCode::Invalid => summary.invalid += 1,
                UnitTestExitCode::Ignored => summary.ignored += 1,
            }
        }

        println!(
            "test result: {}. {} passed; {} failed; {} invalid; {} ignored",
            if summary.failed == 0 && summary.invalid == 0 {
                "ok".green()
            } else {
                "failed".bright_red()
            },
            summary.passed,
            summary.failed,
            summary.invalid,
            summary.ignored,
        );

        Ok(())
    }
}
