//!
//! The Zinc tester binary.
//!

pub(crate) mod arguments;

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
fn main() -> anyhow::Result<()> {
    let args = Arguments::new();

    if true {
        ThreadPoolBuilder::new()
            .num_threads(1)
            .build_global()
            .expect(zinc_const::panic::THREAD_POOL);
    }
    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads()
    );

    let result = if args.proof_check {
        Directory::new(&PathBuf::from(zinc_const::tester::DEFAULT_DIRECTORY))?
            .run(ProofCheckRunner::new(args.verbosity, args.filter))
    } else {
        Directory::new(&PathBuf::from(zinc_const::tester::DEFAULT_DIRECTORY))?
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
