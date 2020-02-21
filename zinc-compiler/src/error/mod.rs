//!
//! The Zinc compiler error.
//!

pub mod file;

use colored::Colorize;

use crate::lexical;
use crate::lexical::Location;
use crate::semantic;
use crate::syntax;

use self::file::Error as FileError;

#[derive(Debug, PartialEq)]
pub enum Error {
    File(FileError),
    Lexical(lexical::Error),
    Syntax(syntax::Error),
    Semantic(semantic::Error),
}

impl Error {
    pub fn format(self, context: &[&str]) -> String {
        match self {
            Self::File(inner) => inner.to_string(),

            Self::Lexical(lexical::Error::UnterminatedBlockComment { start, end }) => {
                Self::format_range(context, "unterminated block comment", start, end)
            }
            Self::Lexical(lexical::Error::UnterminatedDoubleQuoteString { start, end }) => {
                Self::format_range(context, "unterminated double quote string", start, end)
            }
            Self::Lexical(lexical::Error::ExpectedOneOf {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
            ),
            Self::Lexical(lexical::Error::ExpectedOneOfDecimal {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of decimal symbols {}, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
            ),
            Self::Lexical(lexical::Error::ExpectedOneOfHexadecimal {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of hexadecimal symbols {}, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
            ),
            Self::Lexical(lexical::Error::InvalidCharacter { location, found }) => {
                Self::format_line(
                    context,
                    format!("invalid character `{}`", found).as_str(),
                    location,
                )
            }
            Self::Lexical(lexical::Error::UnexpectedEnd { location }) => {
                Self::format_line(context, "unexpected end of input", location)
            }

            Self::Syntax(syntax::Error::ExpectedOneOf {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
            ),
            Self::Syntax(syntax::Error::ExpectedOneOfOrOperator {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of {} or an operator, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
            ),
            Self::Syntax(syntax::Error::ExpectedIdentifier { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected identifier, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedMutOrIdentifier { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected `mut` or identifier, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedFieldIdentifier { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected field identifier, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedType { location, found }) => Self::format_line(
                context,
                format!("expected type, found `{}`", found).as_str(),
                location,
            ),
            Self::Syntax(syntax::Error::ExpectedTypeOrValue { location, found }) => {
                Self::format_line(
                    context,
                    format!(
                        "expected `:` with type or `=` with value, found `{}`",
                        found
                    )
                    .as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedValue { location, found }) => Self::format_line(
                context,
                format!("expected `=` with value, found `{}`", found).as_str(),
                location,
            ),
            Self::Syntax(syntax::Error::ExpectedExpressionOrOperand { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected expression or operand, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedIntegerLiteral { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected integer literal, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedBindingPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected identifier or `_`, found `{}`", found).as_str(),
                    location,
                )
            }
            Self::Syntax(syntax::Error::ExpectedMatchPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!(
                        "expected identifier, boolean or integer literal, path, or `_`, found `{}`",
                        found
                    )
                    .as_str(),
                    location,
                )
            }

            Self::Semantic(inner) => inner.to_string(),
        }
    }

    fn format_line(context: &[&str], message: &str, location: Location) -> String {
        let mut strings = Vec::with_capacity(7);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), location));
        if location.line > 1 {
            strings.push(format!("  {}", "|".bright_cyan()));
        }
        if let Some(line) = context.get(location.line - 1) {
            strings.push(format!(
                "{}{}",
                (location.line.to_string() + " | ").bright_cyan(),
                line
            ));
        }
        strings.push(format!(
            "  {} {}{}",
            "|".bright_cyan(),
            "_".repeat(location.column - 1).bright_red(),
            "^".bright_red()
        ));
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_range(
        context: &[&str],
        message: &'static str,
        start: Location,
        end: Location,
    ) -> String {
        let mut strings = Vec::with_capacity(7 + end.line - start.line);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), start));
        if start.line > 1 {
            strings.push(format!("  {}", "|".bright_cyan()));
        }
        for line_number in start.line..=end.line {
            if let Some(line) = context.get(line_number - 1) {
                strings.push(format!(
                    "{}{}",
                    (line_number.to_string() + " | ").bright_cyan(),
                    line
                ));
            }
        }
        strings.push(format!(
            "  {} {}{}",
            "|".bright_cyan(),
            "_".repeat(end.column - 1).bright_red(),
            "^".bright_red()
        ));
        strings.push(String::new());
        strings.join("\n")
    }
}

impl From<FileError> for Error {
    fn from(error: FileError) -> Self {
        Self::File(error)
    }
}

impl From<lexical::Error> for Error {
    fn from(error: lexical::Error) -> Self {
        Self::Lexical(error)
    }
}

impl From<syntax::Error> for Error {
    fn from(error: syntax::Error) -> Self {
        Self::Syntax(error)
    }
}

impl From<semantic::Error> for Error {
    fn from(error: semantic::Error) -> Self {
        Self::Semantic(error)
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.format(&[])
    }
}
