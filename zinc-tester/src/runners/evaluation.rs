//!
//! The default evaluation test runner.
//!

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;

use pairing::bn256::Bn256;

use crate::data::TestData;
use crate::file::TestFile;
use crate::program::ProgramData;
use crate::runners::TestRunner;
use crate::Summary;

pub struct EvaluationTestRunner {
    pub verbosity: usize,
}

impl TestRunner for EvaluationTestRunner {
    fn run(
        &self,
        test_file_path: &PathBuf,
        test_file: &TestFile,
        test_data: &TestData,
        summary: Arc<Mutex<Summary>>,
    ) {
        let test_file_path = match test_file_path.strip_prefix(crate::TESTS_DIRECTORY) {
            Ok(path) => path,
            Err(_error) => test_file_path,
        };

        for test_case in test_data.cases.iter() {
            let case_name = format!("{}::{}", test_file_path.to_string_lossy(), test_case.case);

            let program_data = match ProgramData::new(&test_case.input, test_file.code.as_str()) {
                Ok(program_data) => program_data,
                Err(error) => {
                    summary.lock().expect(crate::PANIC_SYNC).invalid += 1;
                    println!(
                        "[INTEGRATION] {} {} ({})",
                        "INVALID".red(),
                        case_name,
                        error
                    );
                    continue;
                }
            };

            if test_case.ignore {
                summary.lock().expect(crate::PANIC_SYNC).ignored += 1;
                println!("[INTEGRATION] {} {}", "IGNORE".yellow(), case_name);
                continue;
            }

            match zinc_vm::run::<Bn256>(&program_data.program, &program_data.input) {
                Ok(output) => {
                    let output = output.to_json();
                    if test_case.expect == output {
                        if !test_case.should_panic {
                            summary.lock().expect(crate::PANIC_SYNC).passed += 1;
                            if self.verbosity > 0 {
                                println!("[INTEGRATION] {} {}", "PASSED".green(), case_name);
                            }
                        } else {
                            summary.lock().expect(crate::PANIC_SYNC).failed += 1;
                            println!(
                                "[INTEGRATION] {} {} (should have panicked)",
                                "FAILED".bright_red(),
                                case_name
                            );
                        }
                    } else {
                        summary.lock().expect(crate::PANIC_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (expected {}, but got {})",
                            "FAILED".bright_red(),
                            case_name,
                            test_case.expect,
                            output
                        );
                    }
                }
                Err(error) => {
                    if test_case.should_panic {
                        summary.lock().expect(crate::PANIC_SYNC).passed += 1;
                        if self.verbosity > 0 {
                            println!(
                                "[INTEGRATION] {} {} (panicked)",
                                "PASSED".green(),
                                case_name
                            );
                        }
                    } else {
                        summary.lock().expect(crate::PANIC_SYNC).failed += 1;
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
