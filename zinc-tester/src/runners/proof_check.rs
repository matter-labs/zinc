//!
//! The full proof-check test runner.
//!

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;
use pairing::bn256::Bn256;

use crate::metadata::Metadata;
use crate::file::File;
use crate::program::Program;
use crate::runners::Runnable;
use crate::Summary;

pub struct Runner {
    pub verbosity: usize,
    pub filter: Option<String>,
}

impl Runner {
    pub fn new(verbosity: usize, filter: Option<String>) -> Self {
        Self { verbosity, filter }
    }
}

impl Runnable for Runner {
    fn run(
        &self,
        path: &PathBuf,
        file: &File,
        metadata: &Metadata,
        summary: Arc<Mutex<Summary>>,
    ) {
        let path = match path.strip_prefix(crate::TESTS_DIRECTORY) {
            Ok(path) => path,
            Err(_error) => path,
        };

        for case in metadata.cases.iter() {
            let case_name = format!("{}::{}", path.to_string_lossy(), case.case);
            if let Some(filter) = self.filter.as_ref() {
                if !case_name.contains(filter) {
                    continue;
                }
            }

            if metadata.ignore || case.ignore {
                summary.lock().expect(crate::panic::MUTEX_SYNC).ignored += 1;
                println!("[INTEGRATION] {} {}", "IGNORE".yellow(), case_name);
                continue;
            }

            let program = match Program::new(file.code.as_str(), &case.input, case.entry.as_str()) {
                Ok(program) => program,
                Err(error) => {
                    summary.lock().expect(crate::panic::MUTEX_SYNC).invalid += 1;
                    println!(
                        "[INTEGRATION] {} {} ({})",
                        "INVALID".red(),
                        case_name,
                        error
                    );
                    continue;
                }
            };

            let params = match zinc_vm::setup::<Bn256>(&program.bytecode) {
                Ok(params) => params,
                Err(error) => {
                    summary.lock().expect(crate::panic::MUTEX_SYNC).invalid += 1;
                    println!(
                        "[INTEGRATION] {} {} (setup: {})",
                        "FAILED".red(),
                        path.to_string_lossy(),
                        error
                    );
                    return;
                }
            };

            let (output, proof) = match zinc_vm::prove::<Bn256>(
                &program.bytecode,
                &params,
                &program.witness,
            ) {
                Ok((output, proof)) => {
                    let output_json = output.to_json();
                    if case.expect != output_json {
                        summary.lock().expect(crate::panic::MUTEX_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (expected {}, but got {})",
                            "FAILED".bright_red(),
                            case_name,
                            case.expect,
                            output_json
                        );
                    }
                    (output, proof)
                }
                Err(error) => {
                    if case.should_panic {
                        summary.lock().expect(crate::panic::MUTEX_SYNC).passed += 1;
                        if self.verbosity > 0 {
                            println!(
                                "[INTEGRATION] {} {} (panicked)",
                                "PASSED".green(),
                                case_name
                            );
                        }
                    } else {
                        summary.lock().expect(crate::panic::MUTEX_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (prove: {})",
                            "FAILED".bright_red(),
                            case_name,
                            error
                        );
                    }
                    continue;
                }
            };

            match zinc_vm::verify(&params.vk, &proof, &output) {
                Ok(success) => {
                    if !success {
                        summary.lock().expect(crate::panic::MUTEX_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (verification failed)",
                            "FAILED".bright_red(),
                            case_name
                        );
                    }
                }
                Err(error) => {
                    summary.lock().expect(crate::panic::MUTEX_SYNC).failed += 1;
                    println!(
                        "[INTEGRATION] {} {} (verify: {})",
                        "FAILED".bright_red(),
                        case_name,
                        error
                    );
                }
            }
        }
    }
}
