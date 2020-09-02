//!
//! The compiler executable.
//!

use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;
use std::process::Stdio;

use colored::Colorize;
use failure::Fail;

///
/// The Zinc virtual machine process representation.
///
pub struct VirtualMachine {}

///
/// The Zinc virtual machine process error.
///
#[derive(Debug, Fail)]
pub enum Error {
    /// The process spawning error.
    #[fail(display = "spawning: {}", _0)]
    Spawning(io::Error),
    /// The process stdin getting error.
    #[fail(display = "stdin acquisition")]
    StdinAcquisition,
    /// The process stdout writing error.
    #[fail(display = "stdin writing: {}", _0)]
    StdoutWriting(io::Error),
    /// The process waiting error.
    #[fail(display = "waiting: {}", _0)]
    Waiting(io::Error),
    /// The process returned a non-success exit code.
    #[fail(display = "failure: {}", _0)]
    Failure(ExitStatus),
}

impl VirtualMachine {
    ///
    /// Executes the virtual machine `run` subcommand for circuit.
    ///
    pub fn run_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` {}",
            "Running".bright_green(),
            binary_path.to_string_lossy(),
            if verbosity > 0 {
                format!("-{}", "v".repeat(verbosity))
            } else {
                String::new()
            },
        );

        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("run")
            .arg("--binary")
            .arg(binary_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `run` subcommand for contract.
    ///
    pub fn run_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
        storage_path: &PathBuf,
        method: &str,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` {}",
            "Running".bright_green(),
            binary_path.to_string_lossy(),
            if verbosity > 0 {
                format!("-{}", "v".repeat(verbosity))
            } else {
                String::new()
            },
        );

        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("run")
            .arg("--binary")
            .arg(binary_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--storage")
            .arg(storage_path)
            .arg("--method")
            .arg(method)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `debug` subcommand for circuit.
    ///
    #[allow(dead_code)]
    pub fn debug_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
    ) -> Result<(), Error> {
        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("debug")
            .arg("--binary")
            .arg(binary_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `debug` subcommand for contract.
    ///
    #[allow(dead_code)]
    pub fn debug_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
        storage_path: &PathBuf,
        method: &str,
    ) -> Result<(), Error> {
        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("debug")
            .arg("--binary")
            .arg(binary_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--storage")
            .arg(storage_path)
            .arg("--method")
            .arg(method)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `test` subcommand.
    ///
    pub fn test(verbosity: usize, binary_path: &PathBuf) -> Result<ExitStatus, Error> {
        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("test")
            .arg("--binary")
            .arg(binary_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        Ok(status)
    }

    ///
    /// Executes the virtual machine `setup` subcommand for circuit.
    ///
    pub fn setup_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "  {} key pair `{}` and `{}`",
            "Setting up".bright_green(),
            proving_key_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("setup")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `setup` subcommand for contract.
    ///
    pub fn setup_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        method: &str,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "  {} key pair `{}` and `{}`",
            "Setting up".bright_green(),
            proving_key_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut process = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("setup")
            .arg("--binary")
            .arg(binary_path)
            .arg("--method")
            .arg(method)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = process.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `prove` subcommand for circuit.
    ///
    pub fn prove_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        proving_key_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `prove` subcommand for contract.
    ///
    pub fn prove_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        proving_key_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
        storage_path: &PathBuf,
        method: &str,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--storage")
            .arg(storage_path)
            .arg("--method")
            .arg(method)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `verify` subcommand.
    ///
    pub fn verify_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        verifying_key_path: &PathBuf,
        public_data_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--public-data")
            .arg(public_data_path)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `verify` subcommand.
    ///
    pub fn verify_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        verifying_key_path: &PathBuf,
        public_data_path: &PathBuf,
        method: &str,
    ) -> Result<(), Error> {
        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--method")
            .arg(method)
            .spawn()
            .map_err(Error::Spawning)?;

        let status = child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `prove` and `verify` subcommands for circuit.
    ///
    /// The `prove` command output is passed as the `verify` command input.
    ///
    pub fn prove_and_verify_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let prover_output = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .output()
            .map_err(Error::Spawning)?;

        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut verifier_child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--public-data")
            .arg(public_data_path)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(Error::Spawning)?;
        verifier_child
            .stdin
            .as_mut()
            .ok_or(Error::StdinAcquisition)?
            .write_all(prover_output.stdout.as_slice())
            .map_err(Error::StdoutWriting)?;
        let status = verifier_child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `prove` and `verify` subcommands for contract.
    ///
    /// The `prove` command output is passed as the `verify` command input.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn prove_and_verify_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        witness_path: &PathBuf,
        public_data_path: &PathBuf,
        storage_path: &PathBuf,
        method: &str,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> Result<(), Error> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let prover_output = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--witness")
            .arg(witness_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--storage")
            .arg(storage_path)
            .arg("--method")
            .arg(method)
            .output()
            .map_err(Error::Spawning)?;

        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut verifier_child = process::Command::new(zinc_const::app_name::ZINC_VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--public-data")
            .arg(public_data_path)
            .arg("--method")
            .arg(method)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(Error::Spawning)?;
        verifier_child
            .stdin
            .as_mut()
            .ok_or(Error::StdinAcquisition)?
            .write_all(prover_output.stdout.as_slice())
            .map_err(Error::StdoutWriting)?;
        let status = verifier_child.wait().map_err(Error::Waiting)?;

        if !status.success() {
            return Err(Error::Failure(status));
        }

        Ok(())
    }
}
