//!
//! The Zinc tester binary.
//!

mod data;
mod directory;
mod error;
mod file;
mod program;

use std::convert::TryFrom;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

use colored::Colorize;

use pairing::bn256::Bn256;

use self::data::TestData;
use self::directory::TestDirectory;
use self::error::Error;
use self::file::TestFile;
use self::program::ProgramData;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

fn main() {
    process::exit(match main_inner() {
        Ok(()) => {
            println!("[{}] OK", "INTEGRATION".green());
            EXIT_CODE_SUCCESS
        }
        Err(error) => {
            eprintln!("[{}] {}", "INTEGRATION".red(), error);
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner() -> Result<(), Error> {
    let test_directory = TestDirectory::new(&PathBuf::from("zinc-tester/tests/".to_owned()))
        .map_err(Error::TestDirectory)?;

    let mut is_everything_ok = true;

    for test_file_path in test_directory.file_paths.into_iter() {
        let test_file = TestFile::try_from(&test_file_path).map_err(Error::TestFile)?;
        let test_data = TestData::from_str(test_file.code.as_str())
            .map_err(|error| Error::TestData(test_file_path.as_os_str().to_owned(), error))?;

        for test_case in test_data.cases.into_iter() {
            let program_data = ProgramData::new(&test_case.input, test_file.code.as_str())
                .map_err(Error::ProgramData)?;

            let case_name = format!(
                "{}::{}",
                test_file_path
                    .file_stem()
                    .expect("Test file name")
                    .to_string_lossy(),
                test_case.case
            );

            if test_case.ignore {
                println!("[INTEGRATION] {} {}", "IGNORE".yellow(), case_name);
                continue;
            }

            match zinc_vm::run::<Bn256>(&program_data.program, &program_data.input) {
                Ok(output) => {
                    let output = output.to_json();
                    if test_case.expect == output {
                        if !test_case.should_panic {
                            println!("[INTEGRATION] {} {}", "PASSED".green(), case_name);
                        } else {
                            is_everything_ok = false;
                            println!(
                                "[INTEGRATION] {} {} (should have panicked)",
                                "FAILED".red(),
                                case_name
                            );
                        }
                    } else {
                        is_everything_ok = false;
                        println!(
                            "[INTEGRATION] {} {} (expected {}, but got {})",
                            "FAILED".red(),
                            case_name,
                            test_case.expect,
                            output
                        );
                    }
                }
                Err(error) => {
                    if test_case.should_panic {
                        println!(
                            "[INTEGRATION] {} {} (panicked)",
                            "PASSED".green(),
                            case_name
                        );
                    } else {
                        is_everything_ok = false;
                        println!("[INTEGRATION] {} {} ({})", "FAILED".red(), case_name, error);
                    }
                }
            }
        }
    }

    if is_everything_ok {
        Ok(())
    } else {
        Err(Error::TestFailure)
    }
}
