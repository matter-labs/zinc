use crate::file::TestFile;
use crate::data::TestData;

use std::sync::{Arc, Mutex};
use std::path::PathBuf;

use pairing::bn256::Bn256;
use colored::Colorize;

use crate::program::{ProgramData, compile};
use crate::runners::TestRunner;
use crate::{PANIC_SYNC, Summary, PANIC_TEST_FILE_STEM_GETTING};

pub struct ProofCheckRunner {
    pub verbosity: usize
}

impl TestRunner for ProofCheckRunner {
    fn run(
        &self,
        test_file_path: &PathBuf,
        test_file: &TestFile,
        test_data: &TestData,
        summary: Arc<Mutex<Summary>>
    ) {
        let program = match compile(test_file.code.as_str())
        {
            Ok(program) => program,
            Err(error) => {
                summary.lock().expect(PANIC_SYNC).invalid += 1;
                println!(
                    "[INTEGRATION] {} {} (setup: {})",
                    "INVALID".red(),
                    test_file_path.to_string_lossy(),
                    error
                );
                return;
            }
        };
        let params = match zinc_vm::setup::<Bn256>(&program) {
            Ok(params) => params,
            Err(error) => {
                summary.lock().expect(PANIC_SYNC).invalid += 1;
                println!(
                    "[INTEGRATION] {} {} (setup: {})",
                    "FAILED".red(),
                    test_file_path.to_string_lossy(),
                    error
                );
                return;
            }
        };

        for test_case in test_data.cases.iter() {
            let case_name = format!(
                "{}::{}",
                test_file_path
                    .file_stem()
                    .expect(PANIC_TEST_FILE_STEM_GETTING)
                    .to_string_lossy(),
                test_case.case
            );

            let program_data = match ProgramData::new(&test_case.input, test_file.code.as_str())
            {
                Ok(program_data) => program_data,
                Err(error) => {
                    summary.lock().expect(PANIC_SYNC).invalid += 1;
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
                summary.lock().expect(PANIC_SYNC).ignored += 1;
                println!("[INTEGRATION] {} {}", "IGNORE".yellow(), case_name);
                continue;
            }

            let (output, proof) = match zinc_vm::prove::<Bn256>(&program_data.program, &params, &program_data.input) {
                Ok((output, proof)) => {
                    let output_json = output.to_json();
                    if test_case.expect != output_json {
                        summary.lock().expect(PANIC_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (expected {}, but got {})",
                            "FAILED".bright_red(),
                            case_name,
                            test_case.expect,
                            output_json
                        );
                    }
                    (output, proof)
                }
                Err(error) => {
                    if test_case.should_panic {
                        summary.lock().expect(PANIC_SYNC).passed += 1;
                        if self.verbosity > 0 {
                            println!(
                                "[INTEGRATION] {} {} (panicked)",
                                "PASSED".green(),
                                case_name
                            );
                        }
                    } else {
                        summary.lock().expect(PANIC_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} ({})",
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
                    if success {

                    } else {
                        summary.lock().expect(PANIC_SYNC).failed += 1;
                        println!(
                            "[INTEGRATION] {} {} (verification unsuccessful)",
                            "FAILED".bright_red(),
                            case_name
                        );
                    }
                },
                Err(error) => {
                    summary.lock().expect(PANIC_SYNC).failed += 1;
                    println!(
                        "[INTEGRATION] {} {} (verify: {})",
                        "FAILED".bright_red(),
                        case_name,
                        error
                    );
                },
            }
        }
    }
}
