//!
//! The Zinc logger initializer.
//!

use std::io::Write;

use log::Level;

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
    env_logger::builder()
        .filter_level(match verbosity {
            0 => log::LevelFilter::Warn,
            1 => log::LevelFilter::Info,
            2 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .format(move |buffer, record| {
            if let Level::Debug | Level::Trace = record.level() {
                writeln!(
                    buffer,
                    "[{:>5} {:>5}] {}",
                    level_string(record.level()),
                    record.module_path().unwrap_or_else(|| app_name).white(),
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
fn level_string(level: log::Level) -> ColoredString {
    match level {
        log::Level::Error => "ERROR".bold().red(),
        log::Level::Warn => "WARN".bold().yellow(),
        log::Level::Info => "INFO".bold().blue(),
        log::Level::Debug => "DEBUG".bold().magenta(),
        log::Level::Trace => "TRACE".bold(),
    }
}