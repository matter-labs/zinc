//!
//! The Zinc tester binary.
//!

mod data;
mod directory;
mod file;
mod program;
mod arguments;

use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;
use structopt::StructOpt;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

use pairing::bn256::Bn256;

use self::data::TestData;
use self::directory::TestDirectory;
use self::file::TestFile;
use self::program::ProgramData;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

fn main() {
    process::exit(match main_inner() {
        summary if summary.failed == 0 && summary.invalid == 0 => {
            println!("[{}] {} ({})", "INTEGRATION".green(), "OK".green(), summary);
            EXIT_CODE_SUCCESS
        }
        summary => {
            println!(
                "[{}] {} ({})",
                "INTEGRATION".bright_red(),
                "FAIL".bright_red(),
                summary
            );
            EXIT_CODE_FAILURE
        }
    })
}

static PANIC_TEST_DIRECTORY_INVALID: &str = "The test files directory must be valid";
static PANIC_TEST_FILE_STEM_GETTING: &str = "Every test file must have a stem";

static PANIC_THE_ONLY_REFERENCE: &str =
    "The last shared reference is always unwrapped successfully";
static PANIC_SYNC: &str = "Synchronization is always successful";

fn main_inner() -> Summary {
    let args = arguments::Arguments::from_args();

    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads()
    );

    let summary = Arc::new(Mutex::new(Summary::default()));
    let summary_inner = summary.clone();

    TestDirectory::new(&PathBuf::from("zinc-tester/tests/".to_owned()))
        .expect(PANIC_TEST_DIRECTORY_INVALID)
        .file_paths
        .into_par_iter()
        .map(move |test_file_path| {
            let summary = summary_inner.clone();

            let test_file = TestFile::try_from(&test_file_path).expect(&format!("Test file {:?} is invalid", test_file_path));
            let test_data =
                TestData::from_str(test_file.code.as_str()).expect(&format!("Test file {:?} case data is invalid", test_file_path));

            for test_case in test_data.cases.into_iter() {
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

                match zinc_vm::run::<Bn256>(&program_data.program, &program_data.input) {
                    Ok(output) => {
                        let output = output.to_json();
                        if test_case.expect == output {
                            if !test_case.should_panic {
                                summary.lock().expect(PANIC_SYNC).passed += 1;
                                if !args.quiet {
                                    println!("[INTEGRATION] {} {}", "PASSED".green(), case_name);
                                }
                            } else {
                                summary.lock().expect(PANIC_SYNC).failed += 1;
                                println!(
                                    "[INTEGRATION] {} {} (should have panicked)",
                                    "FAILED".bright_red(),
                                    case_name
                                );
                            }
                        } else {
                            summary.lock().expect(PANIC_SYNC).failed += 1;
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
                            summary.lock().expect(PANIC_SYNC).passed += 1;
                            if !args.quiet {
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
                    }
                }
            }
        })
        .collect::<Vec<()>>();

    Arc::try_unwrap(summary)
        .expect(PANIC_THE_ONLY_REFERENCE)
        .into_inner()
        .expect(PANIC_THE_ONLY_REFERENCE)
}

#[derive(Debug, Default)]
struct Summary {
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
    pub invalid: usize,
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} passed, {} failed, {} ignored, {} invalid",
            self.passed, self.failed, self.ignored, self.invalid
        )
    }
}
