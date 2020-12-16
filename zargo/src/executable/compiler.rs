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
    #[allow(clippy::too_many_arguments)]
    pub fn build_debug(
        verbosity: usize,
        name: &str,
        version: &semver::Version,
        manifest_path: &PathBuf,
        is_test_only: bool,
    ) -> anyhow::Result<()> {
        eprintln!("   {} {} v{}", "Compiling".bright_green(), name, version);

        let mut child = process::Command::new(zinc_const::app_name::COMPILER)
            .args(vec!["-v"; verbosity])
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

        eprintln!("    {} dev [unoptimized] target", "Finished".bright_green());

        Ok(())
    }

    ///
    /// Executes the compiler process, building the release build with optimizations.
    ///
    /// If `is_test_only` is set, passes the flag to only build the project unit tests.
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn build_release(
        verbosity: usize,
        name: &str,
        version: &semver::Version,
        manifest_path: &PathBuf,
        is_test_only: bool,
    ) -> anyhow::Result<()> {
        eprintln!("   {} {} v{}", "Compiling".bright_green(), name, version);

        let mut child = process::Command::new(zinc_const::app_name::COMPILER)
            .args(vec!["-v"; verbosity])
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

        eprintln!(
            "    {} release [optimized] target",
            "Finished".bright_green(),
        );

        Ok(())
    }
}
