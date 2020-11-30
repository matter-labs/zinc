//!
//! The syntax parser error.
//!

use zinc_lexical::Lexeme;
use zinc_lexical::Location;

///
/// The syntax parser error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// One of the common `expected-*` class errors.
    ExpectedOneOf {
        /// The invalid lexeme location.
        location: Location,
        /// The list of the expected lexemes.
        expected: String,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedOneOfOrOperator {
        /// The invalid lexeme location.
        location: Location,
        /// The list of the expected lexemes.
        expected: String,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedIdentifier {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedMutOrIdentifier {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedFieldIdentifier {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedType {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedExpressionOrOperand {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
    },
    /// One of the common `expected-*` class errors.
    ExpectedTypeOrValue {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedValue {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
        /// The optional error hint text.
        help: Option<&'static str>,
    },
    /// One of the common `expected-*` class errors.
    ExpectedLiteral {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
    },
    /// One of the common `expected-*` class errors.
    ExpectedIntegerLiteral {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
    },
    /// One of the common `expected-*` class errors.
    ExpectedBindingPattern {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
    },
    /// One of the common `expected-*` class errors.
    ExpectedMatchPattern {
        /// The invalid lexeme location.
        location: Location,
        /// The invalid lexeme.
        found: Lexeme,
    },
}

///
/// The lexical and syntax errors wrapper.
///
#[derive(Debug, PartialEq)]
pub enum ParsingError {
    /// The lexical analysis error.
    Lexical(zinc_lexical::Error),
    /// The syntax analysis error.
    Syntax(Error),
}

impl From<zinc_lexical::Error> for ParsingError {
    fn from(inner: zinc_lexical::Error) -> Self {
        Self::Lexical(inner)
    }
}

impl Error {
    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of(
        location: Location,
        expected: Vec<&'static str>,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedOneOf {
            location,
            expected: Self::format_one_of(expected.as_slice()),
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_one_of_or_operator(
        location: Location,
        expected: Vec<&'static str>,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedOneOf {
            location,
            expected: Self::format_one_of(expected.as_slice()),
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_identifier(
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedIdentifier {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_mut_or_identifier(
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedMutOrIdentifier {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_field_identifier(
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedFieldIdentifier {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_type(location: Location, found: Lexeme, help: Option<&'static str>) -> Self {
        Self::ExpectedType {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_type_or_value(
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    ) -> Self {
        Self::ExpectedTypeOrValue {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_value(location: Location, found: Lexeme, help: Option<&'static str>) -> Self {
        Self::ExpectedValue {
            location,
            found,
            help,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_expression_or_operand(location: Location, found: Lexeme) -> Self {
        Self::ExpectedExpressionOrOperand { location, found }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_literal(location: Location, found: Lexeme) -> Self {
        Self::ExpectedLiteral { location, found }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_integer_literal(location: Location, found: Lexeme) -> Self {
        Self::ExpectedIntegerLiteral { location, found }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_binding_pattern(location: Location, found: Lexeme) -> Self {
        Self::ExpectedBindingPattern { location, found }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn expected_match_pattern(location: Location, found: Lexeme) -> Self {
        Self::ExpectedMatchPattern { location, found }
    }

    ///
    /// Converts a group of lexemes into a comma-separated list.
    ///
    /// E.g. ["if", "for", "let"] turns into `if`, `for`, `let`.
    ///
    pub fn format_one_of(lexemes: &[&'static str]) -> String {
        lexemes
            .iter()
            .map(|lexeme| format!("`{}`", lexeme))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
