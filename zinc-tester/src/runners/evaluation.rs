//!
//! The default evaluation test runner.
//!

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;
use num::BigInt;
use num::Zero;

use zinc_build::ContractFieldValue as BuildContractFieldValue;
use zinc_vm::Bn256;
use zinc_vm::CircuitFacade;
use zinc_vm::ContractFacade;
use zinc_vm::ContractInput;
use zinc_zksync::TransactionMsg;

use crate::file::File;
use crate::instance::Instance;
use crate::metadata::Metadata;
use crate::runners::IRunnable;
use crate::Summary;

///
/// The evaluation runner.
///
/// Only computes the result, and does not make the trusted setup and proof verification.
///
#[derive(Clone)]
pub struct Runner {
    /// If zero, does not print the successful tests.
    pub verbosity: usize,
    /// If set, runs only the tests whose full names contain the string.
    pub filter: Option<String>,
}

impl Runner {
    ///
    /// Creates a runner instance.
    ///
    pub fn new(verbosity: usize, filter: Option<String>) -> Self {
        Self { verbosity, filter }
    }
}

impl IRunnable for Runner {
    fn run(self, path: PathBuf, file: File, metadata: Metadata, summary: Arc<Mutex<Summary>>) {
        let path = match path.strip_prefix(zinc_const::tester::DEFAULT_DIRECTORY) {
            Ok(path) => path,
            Err(_error) => &path,
        };

        for case in metadata.cases.into_iter() {
            let case_name = format!("{}::{}", path.to_string_lossy(), case.case);
            if let Some(filter) = self.filter.as_ref() {
                if !case_name.contains(filter) {
                    continue;
                }
            }

            if metadata.ignore || case.ignore {
                summary
                    .lock()
                    .expect(zinc_const::panic::SYNCHRONIZATION)
                    .ignored += 1;
                println!("[INTEGRATION] {} {}", "IGNORE".yellow(), case_name);
                continue;
            }

            let mut instance = match Instance::new(
                case_name.clone(),
                file.code.as_str(),
                path.to_owned(),
                case.method.clone(),
                case.input,
            ) {
                Ok(application) => application,
                Err(error) => {
                    summary
                        .lock()
                        .expect(zinc_const::panic::SYNCHRONIZATION)
                        .invalid += 1;
                    println!(
                        "[INTEGRATION] {} {} ({})",
                        "INVALID".red(),
                        case_name,
                        error
                    );
                    continue;
                }
            };

            match instance.application {
                zinc_build::Application::Circuit(circuit) => {
                    let output = CircuitFacade::new(circuit).run::<Bn256>(instance.input);

                    match output {
                        Ok(output) => {
                            let result_json = output.result.into_json();

                            if case.output == result_json {
                                if !case.should_panic {
                                    summary
                                        .lock()
                                        .expect(zinc_const::panic::SYNCHRONIZATION)
                                        .passed += 1;
                                    if self.verbosity > 0 {
                                        println!(
                                            "[INTEGRATION] {} {}",
                                            "PASSED".green(),
                                            case_name
                                        );
                                    }
                                } else {
                                    summary
                                        .lock()
                                        .expect(zinc_const::panic::SYNCHRONIZATION)
                                        .failed += 1;
                                    println!(
                                        "[INTEGRATION] {} {} (should have panicked)",
                                        "FAILED".bright_red(),
                                        case_name
                                    );
                                }
                            } else {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .failed += 1;
                                println!(
                                    "[INTEGRATION] {} {} (expected {}, but got {})",
                                    "FAILED".bright_red(),
                                    case_name,
                                    case.output,
                                    result_json
                                );
                            }
                        }
                        Err(error) => {
                            if case.should_panic {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .passed += 1;
                                if self.verbosity > 0 {
                                    println!(
                                        "[INTEGRATION] {} {} (panicked)",
                                        "PASSED".green(),
                                        case_name
                                    );
                                }
                            } else {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .failed += 1;
                                println!(
                                    "[INTEGRATION] {} {} ({})",
                                    "FAILED".bright_red(),
                                    case_name,
                                    error
                                );
                            }
                        }
                    }
                }
                zinc_build::Application::Contract(contract) => {
                    let storage: Vec<BuildContractFieldValue> = contract
                        .storage
                        .clone()
                        .into_iter()
                        .map(BuildContractFieldValue::new_from_type)
                        .collect();

                    instance.input.insert_contract_instance(BigInt::zero());
                    let output = ContractFacade::new(contract).run::<Bn256>(ContractInput::new(
                        instance.input,
                        zinc_build::Value::Contract(storage),
                        case.method.unwrap_or_else(|| {
                            zinc_const::source::FUNCTION_MAIN_IDENTIFIER.to_owned()
                        }),
                        TransactionMsg::default(),
                    ));

                    match output {
                        Ok(output) => {
                            let result_json = output.result.into_json();

                            if case.output == result_json {
                                if !case.should_panic {
                                    summary
                                        .lock()
                                        .expect(zinc_const::panic::SYNCHRONIZATION)
                                        .passed += 1;
                                    if self.verbosity > 0 {
                                        println!(
                                            "[INTEGRATION] {} {}",
                                            "PASSED".green(),
                                            case_name
                                        );
                                    }
                                } else {
                                    summary
                                        .lock()
                                        .expect(zinc_const::panic::SYNCHRONIZATION)
                                        .failed += 1;
                                    println!(
                                        "[INTEGRATION] {} {} (should have panicked)",
                                        "FAILED".bright_red(),
                                        case_name
                                    );
                                }
                            } else {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .failed += 1;
                                println!(
                                    "[INTEGRATION] {} {} (expected {}, but got {})",
                                    "FAILED".bright_red(),
                                    case_name,
                                    case.output,
                                    result_json
                                );
                            }
                        }
                        Err(error) => {
                            if case.should_panic {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .passed += 1;
                                if self.verbosity > 0 {
                                    println!(
                                        "[INTEGRATION] {} {} (panicked)",
                                        "PASSED".green(),
                                        case_name
                                    );
                                }
                            } else {
                                summary
                                    .lock()
                                    .expect(zinc_const::panic::SYNCHRONIZATION)
                                    .failed += 1;
                                println!(
                                    "[INTEGRATION] {} {} ({})",
                                    "FAILED".bright_red(),
                                    case_name,
                                    error
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
