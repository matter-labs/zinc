use colored::Colorize;
use std::io::Write;

fn level_string(level: log::Level) -> colored::ColoredString {
    match level {
        log::Level::Error => "   error".bold().red(),
        log::Level::Warn => " warning".bold().yellow(),
        log::Level::Info => "    info".bold().blue(),
        log::Level::Debug => "   debug".bold().magenta(),
        log::Level::Trace => "   trace".bold(),
    }
}

fn level_filter_from_verbosity(verbosity: usize) -> log::LevelFilter {
    match verbosity {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    }
}

/// Initialize logger with custom format and verbosity.
///
/// # Arguments
///
/// * `verbosity` - Verbosity level. 0 for `Warn`, 1 for `Info`, 2 for `Debug`, more for `Trace`
pub fn init_logger(verbosity: usize) {
    env_logger::builder()
        .filter_level(level_filter_from_verbosity(verbosity))
        .format(|buf, record| {
            writeln!(
                buf,
                "{}: {}",
                level_string(record.level()),
                record.args().to_string().replace("\n", "\n          ")
            )
        })
        .init();
}
