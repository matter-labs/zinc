//!
//! The compiler executable.
//!

use std::path::PathBuf;
use std::process;

use anyhow::Context;
use colored::Colorize;

use crate::error::Error;

///
/// The Zinc compiler process representation.
///
pub struct Compiler {}

impl Compiler {
    ///
    /// Executes the compiler process, building the debug build without optimizations.
    ///
    /// If `is_test_only` is set, passes the flag to only build the project unit tests.
    ///
    pub fn build_debug(
        verbosity: usize,
        quiet: bool,
        name: &str,
        version: &semver::Version,
        manifest_path: &PathBuf,
        is_test_only: bool,
    ) -> anyhow::Result<()> {
        if !quiet {
            eprintln!("   {} {} v{}", "Compiling".bright_green(), name, version);
        }

        let mut child = process::Command::new(zinc_const::app_name::COMPILER)
            .args(vec!["-v"; verbosity])
            .args(if quiet { vec!["--quiet"] } else { vec![] })
            .arg("--manifest-path")
            .arg(manifest_path)
            .args(if is_test_only {
                vec!["--test-only"]
            } else {
                vec![]
            })
            .spawn()
            .with_context(|| zinc_const::app_name::COMPILER)?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        if !quiet {
            eprintln!("    {} dev [unoptimized] target", "Finished".bright_green());
        }

        Ok(())
    }

    ///
    /// Executes the compiler process, building the release build with optimizations.
    ///
    /// If `is_test_only` is set, passes the flag to only build the project unit tests.
    ///
    pub fn build_release(
        verbosity: usize,
        quiet: bool,
        name: &str,
        version: &semver::Version,
        manifest_path: &PathBuf,
        is_test_only: bool,
    ) -> anyhow::Result<()> {
        if !quiet {
            eprintln!("   {} {} v{}", "Compiling".bright_green(), name, version);
        }

        let mut child = process::Command::new(zinc_const::app_name::COMPILER)
            .args(vec!["-v"; verbosity])
            .args(if quiet { vec!["--quiet"] } else { vec![] })
            .arg("--manifest-path")
            .arg(manifest_path)
            .args(if is_test_only {
                vec!["--test-only"]
            } else {
                vec![]
            })
            .arg("--opt-dfe")
            .spawn()
            .with_context(|| zinc_const::app_name::COMPILER)?;

        let status = child.wait()?;

        if !status.success() {
            anyhow::bail!(Error::SubprocessFailure(status));
        }

        if !quiet {
            eprintln!(
                "    {} release [optimized] target",
                "Finished".bright_green(),
            );
        }

        Ok(())
    }
}
