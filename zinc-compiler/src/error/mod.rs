//!
//! The Zinc compiler error.
//!

pub mod file;

use colored::Colorize;

use crate::lexical::Error as LexicalError;
use crate::lexical::Location;
use crate::semantic::caster::error::Error as CasterError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::scope::error::Error as ScopeError;
use crate::semantic::Error as SemanticError;
use crate::syntax::Error as SyntaxError;

use self::file::Error as FileError;

#[derive(Debug, PartialEq)]
pub enum Error {
    File(FileError),
    Lexical(LexicalError),
    Syntax(SyntaxError),
    Semantic(SemanticError),
}

impl Error {
    pub fn format(self, context: &[&str]) -> String {
        match self {
            Self::File(inner) => inner.to_string(),

            Self::Lexical(LexicalError::UnterminatedBlockComment { start, end }) => {
                Self::format_range(context, "unterminated block comment", start, end, None)
            }
            Self::Lexical(LexicalError::UnterminatedDoubleQuoteString { start, end }) => {
                Self::format_range(
                    context,
                    "unterminated double quote string",
                    start,
                    end,
                    None,
                )
            }
            Self::Lexical(LexicalError::ExpectedOneOf {
                location,
                expected,
                found,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfDecimal {
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
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfHexadecimal {
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
                None,
            ),
            Self::Lexical(LexicalError::InvalidCharacter { location, found }) => Self::format_line(
                context,
                format!("invalid character `{}`", found).as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::UnexpectedEnd { location }) => {
                Self::format_line(context, "unexpected end of input", location, None)
            }

            Self::Syntax(SyntaxError::ExpectedOneOf {
                location,
                expected,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedOneOfOrOperator {
                location,
                expected,
                found,
                help,
            }) => Self::format_line(
                context,
                format!(
                    "expected one of {} or an operator, found `{}`",
                    expected, found
                )
                .as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedMutOrIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected `mut` or identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedFieldIdentifier {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected field identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedType {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected type, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedTypeOrValue {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!(
                    "expected `:` with type or `=` with value, found `{}`",
                    found
                )
                .as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedValue {
                location,
                found,
                help,
            }) => Self::format_line(
                context,
                format!("expected `=` with value, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedExpressionOrOperand { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected expression or operand, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedIntegerLiteral { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected integer literal, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedBindingPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!("expected identifier or `_`, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedMatchPattern { location, found }) => {
                Self::format_line(
                    context,
                    format!(
                        "expected identifier, boolean or integer literal, path, or `_`, found `{}`",
                        found
                    )
                    .as_str(),
                    location,
                    None,
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::FromInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::FromInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::ToInvalidType(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::ToInvalidType(from, to))))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot cast from `{}` to `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("only integer values can be casted to greater or equal bitlength"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Casting(CasterError::ToLesserBitlength(from, to))))) |
            Self::Semantic(SemanticError::Element(location, ElementError::Constant(ConstantError::Casting(CasterError::ToLesserBitlength(from, to))))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot cast an integer with bitlength `{}` to an integer with bitlength `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("integer values can only be casted to greater or equal bitlength"),
                )
            }

            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemRedeclared(item, reference))) => {
                Self::format_line_with_reference(
                    context,
                    format!(
                        "item `{}` redeclared here",
                        item
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    Some("consider giving the latter item another name"),
                )
            }
            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemUndeclared(item))) => {
                Self::format_line(
                    context,
                    format!(
                        "cannot find item `{}` in this scope",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Scope(location, ScopeError::ItemIsNotNamespace(item))) => {
                Self::format_line(
                    context,
                    format!(
                        "item `{}` is not a namespace",
                        item
                    )
                        .as_str(),
                    location,
                    Some("only modules, structures, and enumerations can contain items within their namespaces"),
                )
            }

            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::PushingInvalidType(expected, found))))) => {
                Self::format_line(
                    context,
                    format!(
                        "expected `{}`, found `{}`",
                        expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceStartOutOfRange(left))))) => {
                Self::format_line(
                    context,
                    format!(
                        "left slice bound `{}` is negative",
                        left,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceEndOutOfRange(right, size))))) => {
                Self::format_line(
                    context,
                    format!(
                        "right slice bound `{}` is out of range of the array of size {}",
                        right, size,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(location, ElementError::Value(ValueError::Array(ArrayValueError::SliceEndLesserThanStart(left, right))))) => {
                Self::format_line(
                    context,
                    format!(
                        "left slice bound `{}` is greater than right slice bound `{}`",
                        left, right,
                    )
                        .as_str(),
                    location,
                    Some("left slice range bound must be lesser or equal to the right one"),
                )
            }

            Self::Semantic(SemanticError::MatchNotExhausted(location)) => {
                Self::format_line(
                    context,
                    "match expression must be exhaustive",
                    location,
                    Some("consider covering all possible cases or adding an irrefutable branch with a binding or `_` wildcard"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchUnreachable(location)) => {
                Self::format_line(
                    context,
                    "match expression branch is unreachable",
                    location,
                    Some("consider removing the branch or moving it above the branch with irrefutable pattern"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternPathExpectedEvaluable(location, item)) => {
                Self::format_line(
                    context,
                    format!("expected value, found `{}`", item).as_str(),
                    location,
                    Some("consider specifying a path to an evaluable item"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternInvalidType(location, expected, found, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branch patterns must be compatible with the type of the expression being matched"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchExpressionInvalidType(location, expected, found, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branches must return type equal to the type of the first branch"),
                )
            }

            Self::Semantic(SemanticError::MutatingWithDifferentType(location, expected, found)) => {
                Self::format_line(
                    context,
                    format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::MutatingImmutableMemory(location, name, reference)) => {
                Self::format_line_with_reference(
                    context,
                    format!("cannot assign twice to immutable variable `{}`", name).as_str(),
                    location,
                    reference,
                    Some(format!("make this variable mutable: `mut {}`", name).as_str()),
                )
            }

            Self::Semantic(SemanticError::LoopWhileExpectedBooleanCondition(location, r#type)) => {
                Self::format_line(
                    context,
                    format!("expected `bool`, found `{}`", r#type).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::LoopBoundsExpectedConstantRangeExpression(location, value)) => {
                Self::format_line(
                    context,
                    format!("expected a constant range expression, found `{}`", value).as_str(),
                    location,
                    Some("only constant ranges allowed, e.g. `for i in 0..42 { ... }`"),
                )
            }

            Self::Semantic(SemanticError::ConditionalExpectedBooleanCondition(location, r#type)) => {
                Self::format_line(
                    context,
                    format!("expected `bool`, found `{}`", r#type).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::ConditionalBranchTypesMismatch(location, first, second)) => {
                Self::format_line(
                    context,
                    format!("if and else have incompatible types `{}` and `{}`", first, second).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::EntryPointMissing) => {
                Self::format_message(
                    "function `main` is missing",
                    Some("create the `main` function in the entry point file `main.zn`"),
                )
            }
            Self::Semantic(SemanticError::ModuleNotFound(location, name)) => {
                Self::format_line(
                    context,
                    format!(
                        "file not found for module `{}`",
                        name
                    )
                        .as_str(),
                    location,
                    Some(format!("create a file called `{}.zn` inside the 'src' directory", name).as_str()),
                )
            }
            Self::Semantic(SemanticError::UseExpectedPath(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "`use` expected an item path, but got `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    Some("consider specifying a valid path to an item to import"),
                )
            }
            Self::Semantic(SemanticError::ImplStatementExpectedStructureOrEnumeration(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "`impl` expected a type with namespace, but got `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    Some("only structures and enumerations can have an implementation"),
                )
            }
            Self::Semantic(SemanticError::TypeAliasDoesNotPointToType(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "expected type, found `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::TypeAliasDoesNotPointToStructure(location, item)) => {
                Self::format_line(
                    context,
                    format!(
                        "expected structure type, found `{}`",
                        item
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::ConstantExpressionHasNonConstantElement(location, _item)) => {
                Self::format_line(
                    context,
                    "attempt to use a non-constant value in a constant",
                    location,
                    None,
                )
            }

            Self::Semantic(inner) => inner.to_string(),
        }
    }

    fn format_message(message: &str, help: Option<&str>) -> String {
        let mut strings = Vec::with_capacity(8);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_line(
        context: &[&str],
        message: &str,
        location: Location,
        help: Option<&str>,
    ) -> String {
        let line_number_length = location.line.to_string().len();

        let mut strings = Vec::with_capacity(8);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), location));
        strings.push(format!(
            "{}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan()
        ));
        if let Some(line) = context.get(location.line - 1) {
            strings.push(format!(
                "{}{}",
                (location.line.to_string() + " | ").bright_cyan(),
                line
            ));
        }
        strings.push(format!(
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(location.column - 1).bright_red(),
            "^".bright_red()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_line_with_reference(
        context: &[&str],
        message: &str,
        location: Location,
        reference: Option<Location>,
        help: Option<&str>,
    ) -> String {
        let line_number_length = location.line.to_string().len();

        let mut strings = Vec::with_capacity(11);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));

        if let Some(reference) = reference {
            let line_number_length = reference.line.to_string().len();
            strings.push(format!(
                "{}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan()
            ));
            if let Some(line) = context.get(reference.line - 1) {
                strings.push(format!(
                    "{}{}",
                    (reference.line.to_string() + " | ").bright_cyan(),
                    line
                ));
            }
            strings.push(format!(
                "{}{} {}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan(),
                "_".repeat(reference.column - 1).bright_red(),
                "^".bright_red()
            ));
        }

        strings.push(format!(" {} {}", "-->".bright_cyan(), location));

        if location.line > 1 {
            strings.push(format!(
                "{}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan()
            ));
        }
        if let Some(line) = context.get(location.line - 1) {
            strings.push(format!(
                "{}{}",
                (location.line.to_string() + " | ").bright_cyan(),
                line
            ));
        }
        strings.push(format!(
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(location.column - 1).bright_red(),
            "^".bright_red()
        ));

        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }

    fn format_range(
        context: &[&str],
        message: &'static str,
        start: Location,
        end: Location,
        help: Option<&str>,
    ) -> String {
        let line_number_length = end.line.to_string().len();

        let mut strings = Vec::with_capacity(8 + end.line - start.line);
        strings.push(String::new());
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
        strings.push(format!(" {} {}", "-->".bright_cyan(), start));
        if start.line > 1 {
            strings.push(format!(
                "{}{}",
                " ".repeat(line_number_length + 1),
                "|".bright_cyan()
            ));
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
            "{}{} {}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan(),
            "_".repeat(end.column - 1).bright_red(),
            "^".bright_red()
        ));
        if let Some(help) = help {
            strings.push(format!("{}: {}", "help".bright_white(), help.bright_blue()));
        }
        strings.push(String::new());
        strings.join("\n")
    }
}

impl From<FileError> for Error {
    fn from(error: FileError) -> Self {
        Self::File(error)
    }
}

impl From<LexicalError> for Error {
    fn from(error: LexicalError) -> Self {
        Self::Lexical(error)
    }
}

impl From<SyntaxError> for Error {
    fn from(error: SyntaxError) -> Self {
        Self::Syntax(error)
    }
}

impl From<SemanticError> for Error {
    fn from(error: SemanticError) -> Self {
        Self::Semantic(error)
    }
}

impl Into<String> for Error {
    fn into(self) -> String {
        self.format(&[])
    }
}
