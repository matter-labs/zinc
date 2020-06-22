//!
//! The `test` command.
//!

use std::convert::TryFrom;
use std::path::PathBuf;

use colored::Colorize;
use failure::Fail;
use structopt::StructOpt;

use zinc_const::UnitTestExitCode;

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

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the project and saves its output")]
pub struct Command {
    #[structopt(
        short = "v",
        parse(from_occurrences),
        help = "Shows verbose logs, use multiple times for more verbosity"
    )]
    verbosity: usize,

    #[structopt(
        long = "manifest-path",
        help = "Path to Zargo.toml",
        default_value = "./Zargo.toml"
    )]
    manifest_path: PathBuf,

    #[structopt(
        long = "binary",
        help = "Path to the bytecode test files directory",
        default_value = "./build/test"
    )]
    test_binaries_path: PathBuf,
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "manifest file {}", _0)]
    ManifestFile(ManifestError),
    #[fail(display = "compiler {}", _0)]
    Compiler(CompilerError),
    #[fail(display = "test build directory {}", _0)]
    TestBuildDirectory(TestBuildDirectoryError),
    #[fail(display = "virtual machine {}", _0)]
    VirtualMachine(VirtualMachineError),
    #[fail(display = "unknown exit code")]
    UnknownExitCode(Option<i32>),
}

impl Command {
    pub fn execute(self) -> Result<(), Error> {
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

        Compiler::build_test(
            self.verbosity,
            &data_directory_path,
            &build_directory_path,
            &source_directory_path,
        )
        .map_err(Error::Compiler)?;

        let mut passed = 0;
        let mut failed = 0;
        let mut ignored = 0;

        for binary_path in TestBuildDirectory::files(&manifest_path)
            .map_err(Error::TestBuildDirectory)?
            .into_iter()
        {
            let status = VirtualMachine::test(self.verbosity, &binary_path)
                .map_err(Error::VirtualMachine)?;
            let code = UnitTestExitCode::try_from(status).map_err(Error::UnknownExitCode)?;

            match code {
                UnitTestExitCode::Passed => passed += 1,
                UnitTestExitCode::Ignored => ignored += 1,
                UnitTestExitCode::Failed => failed += 1,
            }
        }

        println!(
            "test result: {}. {} passed; {} failed; {} ignored",
            if failed == 0 {
                "ok".green()
            } else {
                "failed".bright_red()
            },
            passed,
            failed,
            ignored
        );

        Ok(())
    }
}
