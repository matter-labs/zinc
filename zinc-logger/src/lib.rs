//!
//! The Zinc logger.
//!

use std::io::Write;

use log::Level;
use log::LevelFilter;

use colored::ColoredString;
use colored::Colorize;

/// The space for the logging level name.
const LEVEL_NAME_LENGTH: usize = 10;

///
/// Initialize logger with custom format and verbosity.
///
/// # Arguments
///
/// Verbosity:
/// 0 for `Warn`,
/// 1 for `Info`,
/// 2 for `Debug`,
/// _ for `Trace`
///
pub fn initialize(app_name: &'static str, verbosity: usize) {
    let level = match verbosity {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    env_logger::builder()
        .filter(None, LevelFilter::Off)
        .filter_module("actix_server", LevelFilter::Info)
        .filter_module(zinc_const::app_name::ZARGO, level)
        .filter_module(zinc_const::app_name::ZANDBOX, level)
        .filter_module(zinc_const::app_name::COMPILER, level)
        .filter_module(zinc_const::app_name::VIRTUAL_MACHINE, level)
        .filter_module(zinc_const::app_name::TESTER, level)
        .filter_module("zargo", level)
        .filter_module("zandbox", level)
        .filter_module("zinc_compiler", level)
        .filter_module("zinc_vm", level)
        .filter_module("zinc_tester", level)
        .format(move |buffer, record| {
            if record.level() >= Level::Debug {
                writeln!(
                    buffer,
                    "[{:>5} {:>5}] {}",
                    level_string(record.level()),
                    record.module_path().unwrap_or(app_name).white(),
                    record.args()
                )
            } else {
                writeln!(
                    buffer,
                    "[{:>5} {:>5}] {}",
                    level_string(record.level()),
                    app_name.white(),
                    format!(
                        "{}{}",
                        record.args(),
                        " ".repeat(app_name.len() + LEVEL_NAME_LENGTH + 4)
                    )
                )
            }
        })
        .init();
}

///
/// The log level string printed to the terminal.
///
fn level_string(level: Level) -> ColoredString {
    match level {
        Level::Error => "ERROR".bold().red(),
        Level::Warn => "WARN".bold().yellow(),
        Level::Info => "INFO".bold().blue(),
        Level::Debug => "DEBUG".bold().magenta(),
        Level::Trace => "TRACE".bold(),
    }
}
