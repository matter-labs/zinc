//!
//! The Zinc tester binary.
//!

mod arguments;
mod panic;

use std::path::PathBuf;
use std::process;

use colored::Colorize;
use rayon::ThreadPoolBuilder;

use zinc_tester::Directory;
use zinc_tester::EvaluationRunner;
use zinc_tester::ProofCheckRunner;

use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() {
    let args = Arguments::new();

    if args.proof_check {
        ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .expect(self::panic::RAYON_POOL_INITIALIZATION);
    }
    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads()
    );

    let result = if args.proof_check {
        Directory::new(&PathBuf::from(zinc_tester::TEST_DEFAULT_DIRECTORY))
            .expect(self::panic::TEST_DIRECTORY_INVALID)
            .run(ProofCheckRunner::new(args.verbosity, args.filter))
    } else {
        Directory::new(&PathBuf::from(zinc_tester::TEST_DEFAULT_DIRECTORY))
            .expect(self::panic::TEST_DIRECTORY_INVALID)
            .run(EvaluationRunner::new(args.verbosity, args.filter))
    };

    match result {
        summary if summary.failed == 0 && summary.invalid == 0 => {
            println!(
                "[{}] {} ({})",
                "INTEGRATION".green(),
                "PASSED".green(),
                summary
            );
            process::exit(zinc_const::exit_code::SUCCESS);
        }
        summary => {
            println!(
                "[{}] {} ({})",
                "INTEGRATION".bright_red(),
                "FAILED".bright_red(),
                summary
            );
            process::exit(zinc_const::exit_code::FAILURE);
        }
    }
}
