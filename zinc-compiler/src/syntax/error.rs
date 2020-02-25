//!
//! The syntax error.
//!

use crate::lexical::Lexeme;
use crate::lexical::Location;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedOneOf {
        location: Location,
        expected: String,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedOneOfOrOperator {
        location: Location,
        expected: String,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedIdentifier {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedMutOrIdentifier {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedFieldIdentifier {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedType {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedExpressionOrOperand {
        location: Location,
        found: Lexeme,
    },
    ExpectedTypeOrValue {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedValue {
        location: Location,
        found: Lexeme,
        help: Option<&'static str>,
    },
    ExpectedIntegerLiteral {
        location: Location,
        found: Lexeme,
    },
    ExpectedBindingPattern {
        location: Location,
        found: Lexeme,
    },
    ExpectedMatchPattern {
        location: Location,
        found: Lexeme,
    },
}

impl Error {
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

    pub fn expected_type(location: Location, found: Lexeme, help: Option<&'static str>) -> Self {
        Self::ExpectedType {
            location,
            found,
            help,
        }
    }

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

    pub fn expected_value(location: Location, found: Lexeme, help: Option<&'static str>) -> Self {
        Self::ExpectedValue {
            location,
            found,
            help,
        }
    }

    pub fn expected_expression_or_operand(location: Location, found: Lexeme) -> Self {
        Self::ExpectedExpressionOrOperand { location, found }
    }

    pub fn expected_integer_literal(location: Location, found: Lexeme) -> Self {
        Self::ExpectedIntegerLiteral { location, found }
    }

    pub fn expected_binding_pattern(location: Location, found: Lexeme) -> Self {
        Self::ExpectedBindingPattern { location, found }
    }

    pub fn expected_match_pattern(location: Location, found: Lexeme) -> Self {
        Self::ExpectedMatchPattern { location, found }
    }

    pub fn format_one_of(lexemes: &[&'static str]) -> String {
        lexemes
            .iter()
            .map(|lexeme| format!("`{}`", lexeme))
            .collect::<Vec<String>>()
            .join(", ")
    }
}
