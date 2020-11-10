//!
//! The compiler executable.
//!

use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::process::ExitStatus;
use std::process::Stdio;

use colored::Colorize;

use crate::error::Error;

///
/// The Zinc virtual machine process representation.
///
pub struct VirtualMachine {}

impl VirtualMachine {
    ///
    /// Executes the virtual machine `run` subcommand for circuit.
    ///
    pub fn run_circuit(
        verbosity: usize,
        binary_path: &PathBuf,
        input_path: &PathBuf,
        output_path: &PathBuf,
    ) -> anyhow::Result<()> {
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

        let mut process = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("run")
            .arg("--binary")
            .arg(binary_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .spawn()?;

        let status = process.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `run` subcommand for contract.
    ///
    pub fn run_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        input_path: &PathBuf,
        output_path: &PathBuf,
        method: &str,
    ) -> anyhow::Result<()> {
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

        let mut process = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("run")
            .arg("--binary")
            .arg(binary_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .arg("--method")
            .arg(method)
            .spawn()?;

        let status = process.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `test` subcommand.
    ///
    pub fn test(verbosity: usize, binary_path: &PathBuf) -> anyhow::Result<ExitStatus> {
        let mut process = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("test")
            .arg("--binary")
            .arg(binary_path)
            .spawn()?;

        let status = process.wait()?;

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
    ) -> anyhow::Result<()> {
        eprintln!(
            "  {} key pair `{}` and `{}`",
            "Setting up".bright_green(),
            proving_key_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut process = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("setup")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .spawn()?;

        let status = process.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
    ) -> anyhow::Result<()> {
        eprintln!(
            "  {} key pair `{}` and `{}`",
            "Setting up".bright_green(),
            proving_key_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut process = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
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
            .spawn()?;

        let status = process.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
        input_path: &PathBuf,
        output_path: &PathBuf,
    ) -> anyhow::Result<()> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
        input_path: &PathBuf,
        output_path: &PathBuf,
        method: &str,
    ) -> anyhow::Result<()> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .arg("--method")
            .arg(method)
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
        output_path: &PathBuf,
    ) -> anyhow::Result<()> {
        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--output")
            .arg(output_path)
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
        output_path: &PathBuf,
        method: &str,
    ) -> anyhow::Result<()> {
        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--output")
            .arg(output_path)
            .arg("--method")
            .arg(method)
            .spawn()?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
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
        input_path: &PathBuf,
        output_path: &PathBuf,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> anyhow::Result<()> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let prover_output = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .output()?;

        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut verifier_child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--output")
            .arg(output_path)
            .stdin(Stdio::piped())
            .spawn()?;
        verifier_child
            .stdin
            .as_mut()
            .ok_or(Error::StdinAcquisition)?
            .write_all(prover_output.stdout.as_slice())?;
        let status = verifier_child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        Ok(())
    }

    ///
    /// Executes the virtual machine `prove` and `verify` subcommands for contract.
    ///
    /// The `prove` command output is passed as the `verify` command input.
    ///
    pub fn prove_and_verify_contract(
        verbosity: usize,
        binary_path: &PathBuf,
        input_path: &PathBuf,
        output_path: &PathBuf,
        method: &str,
        proving_key_path: &PathBuf,
        verifying_key_path: &PathBuf,
    ) -> anyhow::Result<()> {
        eprintln!(
            "     {} `{}` with `{}`",
            "Proving".bright_green(),
            binary_path.to_string_lossy(),
            proving_key_path.to_string_lossy(),
        );

        let prover_output = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("prove")
            .arg("--binary")
            .arg(binary_path)
            .arg("--proving-key")
            .arg(proving_key_path)
            .arg("--input")
            .arg(input_path)
            .arg("--output")
            .arg(output_path)
            .arg("--method")
            .arg(method)
            .output()?;

        eprintln!(
            "   {} `{}` with `{}`",
            "Verifying".bright_green(),
            binary_path.to_string_lossy(),
            verifying_key_path.to_string_lossy(),
        );

        let mut verifier_child = process::Command::new(zinc_const::app_name::VIRTUAL_MACHINE)
            .args(vec!["-v"; verbosity])
            .arg("verify")
            .arg("--binary")
            .arg(binary_path)
            .arg("--verifying-key")
            .arg(verifying_key_path)
            .arg("--output")
            .arg(output_path)
            .arg("--method")
            .arg(method)
            .stdin(Stdio::piped())
            .spawn()?;
        verifier_child
            .stdin
            .as_mut()
            .ok_or(Error::StdinAcquisition)?
            .write_all(prover_output.stdout.as_slice())?;
        let status = verifier_child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        Ok(())
    }
}
