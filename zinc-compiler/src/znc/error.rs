//!
//! The Zinc compiler binary error.
//!

use std::fmt;

use zinc_compiler::SourceError;

pub enum Error {
    Source(SourceError),
    WitnessTemplateOutput(OutputError),
    PublicDataTemplateOutput(OutputError),
    BytecodeOutput(OutputError),
}

impl From<SourceError> for Error {
    fn from(inner: SourceError) -> Self {
        Self::Source(inner)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(inner) => write!(f, "{}", inner),
            Self::WitnessTemplateOutput(inner) => write!(f, "witness template output: {}", inner),
            Self::PublicDataTemplateOutput(inner) => {
                write!(f, "public data template output: {}", inner)
            }
            Self::BytecodeOutput(inner) => write!(f, "bytecode output: {}", inner),
        }
    }
}

pub enum OutputError {
    Creating(std::io::Error),
    Writing(std::io::Error),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Creating(inner) => write!(f, "creating: {}", inner),
            Self::Writing(inner) => write!(f, "writing: {}", inner),
        }
    }
}
