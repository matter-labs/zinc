//!
//! The Zinc tester binary.
//!

mod arguments;

use std::path::PathBuf;
use std::process;

use colored::Colorize;
use rayon::ThreadPoolBuilder;

use zinc_tester::Directory;
use zinc_tester::EvaluationRunner;
use zinc_tester::ProofCheckRunner;

use self::arguments::Arguments;

const EXIT_CODE_SUCCESS: i32 = 0;
const EXIT_CODE_FAILURE: i32 = 1;

static TEST_DIRECTORY_INVALID: &str = "The test files directory must be valid";
static RAYON_POOL_INITIALIZATION: &str = "The thread pool is initialized only once";

fn main() {
    let args = Arguments::new();

    if args.proof_check {
        ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .expect(RAYON_POOL_INITIALIZATION);
    }
    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads()
    );

    let result = if args.proof_check {
        Directory::new(&PathBuf::from(zinc_tester::TESTS_DIRECTORY))
            .expect(TEST_DIRECTORY_INVALID)
            .run(ProofCheckRunner::new(args.verbosity, args.filter))
    } else {
        Directory::new(&PathBuf::from(zinc_tester::TESTS_DIRECTORY))
            .expect(TEST_DIRECTORY_INVALID)
            .run(EvaluationRunner::new(args.verbosity, args.filter))
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
