//!
//! The Zinc tester binary.
//!

pub(crate) mod arguments;

use std::path::PathBuf;
use std::process;

use colored::Colorize;

use self::arguments::Arguments;

///
/// The application entry point.
///
fn main() -> anyhow::Result<()> {
    let args = Arguments::new();

    println!(
        "[INTEGRATION] Started with {} worker threads",
        rayon::current_num_threads(),
    );

    let summary = zinc_tester::Summary::default().wrap();

    println!("[INTEGRATION] Running one-file tests");
    zinc_tester::OneFileTestsDirectory::new(&PathBuf::from(zinc_tester::ONE_FILE_TESTS_DIRECTORY))?
        .run(
            zinc_tester::EvaluationRunner::new(args.verbosity, args.filter),
            summary.clone(),
        );

    println!("[INTEGRATION] Running project tests");
    zinc_tester::OrdinarTestsDirectory::new(&PathBuf::from(
        zinc_tester::ORDINAR_PROJECTS_DIRECTORY,
    ))?
    .run(args.verbosity, summary.clone());

    match zinc_tester::Summary::unwrap_arc(summary) {
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
