//!
//! The Zinc tester binary.
//!

mod arguments;
mod data;
mod directory;
mod file;
mod program;
mod runners;

use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;
use std::process;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use colored::Colorize;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use structopt::StructOpt;

use self::data::TestData;
use self::directory::TestDirectory;
use self::file::TestFile;
use self::runners::EvaluationTestRunner;
use self::runners::ProofCheckRunner;
use self::runners::TestRunner;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

static TESTS_DIRECTORY: &str = "zinc-tester/tests/";

static PANIC_TEST_DIRECTORY_INVALID: &str = "The test files directory must be valid";
static PANIC_LAST_SHARED_REFERENCE: &str = "There are no other references at this point";
static PANIC_MUTEX_SYNC: &str = "Mutexes never panic";

fn main() {
    let args = arguments::Arguments::from_args();
    let result = if args.proof_check {
        let runner = ProofCheckRunner {
            verbosity: args.verbosity,
        };
        main_inner(runner)
    } else {
        let runner = EvaluationTestRunner {
            verbosity: args.verbosity,
        };
        main_inner(runner)
    };

    process::exit(match result {
        summary if summary.failed == 0 && summary.invalid == 0 => {
            println!(
                "[{}] {} ({})",
                "INTEGRATION".green(),
                "PASSED".green(),
                summary
            );
            EXIT_CODE_SUCCESS
        }
        summary => {
            println!(
                "[{}] {} ({})",
                "INTEGRATION".bright_red(),
                "FAILED".bright_red(),
                summary
            );
            EXIT_CODE_FAILURE
        }
    })
}

fn main_inner<R: TestRunner>(runner: R) -> Summary {
    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads()
    );

    let summary = Arc::new(Mutex::new(Summary::default()));

    TestDirectory::new(&PathBuf::from(TESTS_DIRECTORY))
        .expect(PANIC_TEST_DIRECTORY_INVALID)
        .file_paths
        .into_par_iter()
        .map(|test_file_path| {
            let test_file = TestFile::try_from(&test_file_path)
                .unwrap_or_else(|_| panic!("Test file {:?} is invalid", test_file_path));
            let test_data = TestData::from_str(test_file.code.as_str())
                .unwrap_or_else(|_| panic!("Test file {:?} case data is invalid", test_file_path));

            runner.run(&test_file_path, &test_file, &test_data, summary.clone());
        })
        .collect::<Vec<()>>();

    Arc::try_unwrap(summary)
        .expect(PANIC_LAST_SHARED_REFERENCE)
        .into_inner()
        .expect(PANIC_LAST_SHARED_REFERENCE)
}

#[derive(Debug, Default)]
pub struct Summary {
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
