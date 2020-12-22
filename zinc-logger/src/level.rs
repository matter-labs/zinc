//!
//! The Zinc logger level.
//!

use log::LevelFilter;

///
/// The Zinc logger level.
///
pub enum Level {
    /// The `ERROR` log level. Cannot be disabled without the `quiet` flag.
    Error = -1,
    /// The `WARN` log level. Cannot be disabled without the `quiet` flag.
    Warn = 0,
    /// The `INFO` log level.
    Info = 1,
    /// The `DEBUG` log level.
    Debug = 2,
    /// The `TRACE` log level.
    Trace = 3,
}

impl From<usize> for Level {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Warn,
            1 => Self::Info,
            2 => Self::Debug,
            _ => Self::Trace,
        }
    }
}

impl Into<LevelFilter> for Level {
    fn into(self) -> LevelFilter {
        match self {
            Self::Error => LevelFilter::Error,
            Self::Warn => LevelFilter::Warn,
            Self::Info => LevelFilter::Info,
            Self::Debug => LevelFilter::Debug,
            Self::Trace => LevelFilter::Trace,
        }
    }
}

impl Into<usize> for Level {
    fn into(self) -> usize {
        match self {
            Self::Error | Self::Warn => 0,
            Self::Info => 1,
            Self::Debug => 2,
            Self::Trace => 3,
        }
    }
}
