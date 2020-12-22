//!
//! The Zinc compiler error.
//!

use colored::Colorize;

use zinc_lexical::Error as LexicalError;
use zinc_lexical::Location;
use zinc_lexical::FILE_INDEX;
use zinc_syntax::Error as SyntaxError;
use zinc_syntax::ParsingError;

use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::error::Error as SemanticError;

///
/// The Zinc compiler error.
///
#[derive(Debug, PartialEq)]
pub enum Error {
    /// The lexical analysis error.
    Lexical(LexicalError),
    /// The syntax analysis error.
    Syntax(SyntaxError),
    /// The semantic analysis error.
    Semantic(SemanticError),
}

impl Error {
    ///
    /// Formats the compiler error into the user-friendly readable output.
    ///
    pub fn format(self) -> String {
        let code = self.code();

        match self {
            Self::Lexical(LexicalError::UnterminatedBlockComment { start, end }) => {
                Self::format_range("unterminated block comment", code,start, end, None)
            }
            Self::Lexical(LexicalError::UnterminatedDoubleQuoteString { start, end }) => {
                Self::format_range(
                    "unterminated double quote string",
                    code,
                    start,
                    end,
                    None,
                )
            }
            Self::Lexical(LexicalError::ExpectedOneOfBinary {
                              location,
                              expected,
                              found,
                          }) => Self::format_line( format!(
                    "expected one of binary symbols {} or '_', found `{}`",
                    expected, found
                )
                    .as_str(),
                                                   code,location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfOctal {
                              location,
                              expected,
                              found,
                          }) => Self::format_line( format!(
                    "expected one of octal symbols {} or '_', found `{}`",
                    expected, found
                )
                    .as_str(),
                                                   code,location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfDecimal {
                location,
                expected,
                found,
            }) => Self::format_line( format!(
                    "expected one of decimal symbols {} or '_', found `{}`",
                    expected, found
                )
                .as_str(),
                                     code,location,
                None,
            ),
            Self::Lexical(LexicalError::ExpectedOneOfHexadecimal {
                location,
                expected,
                found,
            }) => Self::format_line( format!(
                    "expected one of hexadecimal symbols {} or '_', found `{}`",
                    expected, found
                )
                .as_str(),
                                     code,location,
                None,
            ),
            Self::Lexical(LexicalError::InvalidCharacter { location, found }) => Self::format_line( format!("invalid character `{}`", found).as_str(),
                                                                                                    code,location,
                None,
            ),
            Self::Lexical(LexicalError::UnexpectedEnd { location }) => {
                Self::format_line( "unexpected end of input", code, location, None)
            }
            Self::Syntax(SyntaxError::ExpectedOneOf {
                location,
                expected,
                found,
                help,
            }) => Self::format_line( format!("expected one of {}, found `{}`", expected, found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedOneOfOrOperator {
                location,
                expected,
                found,
                help,
            }) => Self::format_line( format!(
                    "expected one of {} or an operator, found `{}`",
                    expected, found
                )
                .as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected identifier, found `{}`", found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedMutOrIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected keyword `mut` or identifier, found `{}`", found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedFieldIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected field identifier, found `{}`", found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedType {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected type, found `{}`", found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedTypeOrValue {
                location,
                found,
                help,
            }) => Self::format_line( format!(
                    "expected `:` with type or `=` with value, found `{}`",
                    found
                )
                .as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedValue {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected `=` with value, found `{}`", found).as_str(),
                code,location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedExpressionOrOperand { location, found }) => {
                Self::format_line( format!("expected expression or operand, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedLiteral { location, found }) => {
                Self::format_line( format!("expected literal, found `{}`", found).as_str(),
                                   code,location,
                                   None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedIntegerLiteral { location, found }) => {
                Self::format_line( format!("expected integer literal, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedBindingPattern { location, found }) => {
                Self::format_line( format!("expected identifier or `_`, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedMatchPattern { location, found }) => {
                Self::format_line( format!(
                        "expected identifier, boolean or integer literal, path, or `_`, found `{}`",
                        found
                    )
                    .as_str(),
                    code,location,
                None,
                )
            }

            Self::Semantic(SemanticError::InvalidInteger { location, inner: zinc_math::Error::NumberParsing(inner) }) => {
                Self::format_line(format!("The number parsing error: {}", inner).as_str(),
                                  code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::InvalidInteger { location, inner: zinc_math::Error::ExponentParsing(inner) }) => {
                Self::format_line(format!("The exponent value parsing error: {}", inner).as_str(),
                                  code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::InvalidInteger { location, inner: zinc_math::Error::Overflow { value, is_signed, bitlength } }) => {
                Self::format_line( format!("`{}` is larger than `{}` bits with sign `{}`", value, bitlength, is_signed).as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::InvalidInteger { location, inner: zinc_math::Error::ExponentTooSmall(exponent) }) => {
                Self::format_line(format!("The exponent value `{}` is too small", exponent).as_str(),
                                  code, location,
                                   Some("the exponent value must be equal or greater than the number of fractional digits"),
                )
            }

            Self::Semantic(SemanticError::OperatorAssignmentFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise OR operator `|=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise OR operator `|=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise XOR operator `^=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise XOR operator `^=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise AND operator `&=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise AND operator `&=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise shift left operator `<<=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise shift left operator `<<=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise shift right operator `>>=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment bitwise shift right operator `>>=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentAdditionFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `+=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `+=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentSubtractionFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `-=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `-=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `*=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `*=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentDivisionFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `/=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `/=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentRemainderFirstOperandExpectedPlace{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `%=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the assignment operator `%=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRangeInclusiveFirstOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorRangeInclusiveFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the inclusive range operator `..=` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRangeInclusiveSecondOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorRangeInclusiveSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the inclusive range operator `..=` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRangeFirstOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorRangeFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the range operator `..` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRangeSecondOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorRangeSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the range operator `..` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorOrFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorOrFirstOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the OR operator `||` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorOrSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorOrSecondOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the OR operator `||` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorXorFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorXorFirstOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the XOR operator `^^` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorXorSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorXorSecondOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the XOR operator `^^` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAndFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorAndFirstOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the AND operator `&&` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAndSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorAndSecondOperandExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the AND operator `&&` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorEqualsFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorEqualsFirstOperandExpectedPrimitiveType{ location, found }) => {
                Self::format_line( format!(
                        "the equals operator `==` expected a unit, boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorEqualsSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorEqualsSecondOperandExpectedUnit{ location, found }) |
            Self::Semantic(SemanticError::OperatorEqualsSecondOperandExpectedBoolean{ location, found }) |
            Self::Semantic(SemanticError::OperatorEqualsSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the equals operator `==` expected a unit, boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorEqualsTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the equals operator `==` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNotEqualsFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorNotEqualsFirstOperandExpectedPrimitiveType{ location, found }) => {
                Self::format_line( format!(
                        "the not equals operator `!=` expected a boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNotEqualsSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorNotEqualsSecondOperandExpectedUnit{ location, found }) |
            Self::Semantic(SemanticError::OperatorNotEqualsSecondOperandExpectedBoolean{ location, found }) |
            Self::Semantic(SemanticError::OperatorNotEqualsSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the not equals operator `!=` expected a boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNotEqualsTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the not equals operator `!=` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterEqualsFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorGreaterEqualsFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the greater equals operator `>=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterEqualsSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorGreaterEqualsSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the greater equals operator `>=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterEqualsTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the greater equals operator `>=` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserEqualsFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorLesserEqualsFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the lesser equals operator `<=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserEqualsSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorLesserEqualsSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the lesser equals operator `<=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserEqualsTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the lesser equals operator `<=` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorGreaterFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the greater operator `>` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorGreaterSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the greater operator `>` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorGreaterTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the greater operator `>` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorLesserFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the lesser operator `<` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorLesserSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the lesser operator `<` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorLesserTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the lesser operator `<` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseOrFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseOrFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise OR operator `|` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseOrSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseOrSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise OR operator `|` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseOrTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the bitwise OR operator `|` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseXorFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseXorFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise XOR operator `^` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseXorSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseXorSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise XOR operator `^` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseXorTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the bitwise XOR operator `^` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseAndFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseAndFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise AND operator `&` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseAndSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseAndSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise AND operator `&` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseAndTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the bitwise AND operator `&` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise shift left operator `<<` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseShiftLeftSecondOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned { location, found }) => {
                Self::format_line( format!(
                        "the bitwise shift left operator `<<` expected an unsigned integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftRightFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise shift right operator `>>` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseShiftRightSecondOperandExpectedConstant{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftRightSecondOperandExpectedInteger{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned { location, found }) => {
                Self::format_line( format!(
                        "the bitwise shift right operator `>>` expected an unsigned integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseSignedOperandForbidden { location }) => {
                Self::format_line( "the bitwise operators are forbidden for the signed integer types",
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseFieldOperandForbidden { location }) => {
                Self::format_line( "the bitwise operators are forbidden for the `field` type",
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAdditionFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorAdditionFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the addition operator `+` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAdditionSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorAdditionSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the addition operator `+` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAdditionTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the addition operator `+` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorAdditionOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the addition operator `+` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorSubtractionFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorSubtractionFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the subtraction operator `-` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorSubtractionSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorSubtractionSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the subtraction operator `-` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorSubtractionTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the subtraction operator `-` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorSubtractionOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the subtraction operator `-` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorMultiplicationFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorMultiplicationFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the multiplication operator `*` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorMultiplicationSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorMultiplicationSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the multiplication operator `*` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorMultiplicationTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the multiplication operator `*` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorMultiplicationOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the multiplication operator `*` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorDivisionFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the division operator `/` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorDivisionSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the division operator `/` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the division operator `/` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the division operator `/` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionFieldOperandForbidden { location }) => {
                Self::format_line( "the division operator `/` is forbidden for the `field` type",
                                   code, location,
                                   Some("for inversion consider using `std::ff::invert`"),
                )
            }
            Self::Semantic(SemanticError::OperatorDivisionByZero { location }) => {
                Self::format_line( "division by zero",
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderFirstOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorRemainderFirstOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the remainder operator `%` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorRemainderSecondOperandExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the remainder operator `%` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderTypesMismatch { location, first, second }) => {
                Self::format_line( format!(
                    "the remainder operator `%` expected two integers of the same type, found `{}` and `{}`",
                    first, second,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the remainder operator `%` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderFieldOperandForbidden { location }) => {
                Self::format_line( "the remainder operator `%` is forbidden for the `field` type",
                                   code, location,
                                   Some("`field` type values cannot be used to get a remainder"),
                )
            }
            Self::Semantic(SemanticError::OperatorRemainderOfDivisionByZero { location }) => {
                Self::format_line( "remainder of division by zero",
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorCastingFirstOperandExpectedEvaluable{ location, found }) => {
                Self::format_line( format!(
                        "the casting operator `as` expected a value as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorCastingSecondOperandExpectedType{ location, found }) => {
                Self::format_line( format!(
                        "the casting operator `as` expected a type as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorCastingTypesMismatch { location, inner: CastingError::CastingFromInvalidType { from, to }, reference }) |
            Self::Semantic(SemanticError::OperatorCastingTypesMismatch { location, inner: CastingError::CastingToInvalidType { from, to }, reference }) => {
                Self::format_line_with_reference(format!(
                        "cannot cast from `{}` to `{}`",
                        from, to,
                    )
                        .as_str(),
                    code, location,
                                   Some(reference),
                    Some("only integer values can be casted to an integer with different bitlength or field element"),
                )
            }
            Self::Semantic(SemanticError::OperatorCastingOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the casting operator `as` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNotExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorNotExpectedBoolean{ location, found }) => {
                Self::format_line( format!(
                        "the NOT operator `!` expected a boolean, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorBitwiseNotExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorBitwiseNotExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the bitwise NOT operator `~` expected an integer, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNegationExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorNegationExpectedInteger{ location, found }) => {
                Self::format_line( format!(
                        "the negation operator `-` expected an integer, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNegationOverflow { location, value, r#type }) => {
                Self::format_line( format!(
                    "the negation operator `-` overflow, as the value `{}` cannot be represeneted by type `{}`",
                    value, r#type,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorNegationFieldOperandForbidden { location }) => {
                Self::format_line( "the negation operator `-` is forbidden for the `field` type",
                                   code, location,
                                   Some("`field` type values cannot be negative"),
                )
            }
            Self::Semantic(SemanticError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorIndexFirstOperandExpectedArray{ location, found }) => {
                Self::format_line( format!(
                        "the index operator `[]` expected an array as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorIndexSecondOperandExpectedEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorIndexSecondOperandExpectedIntegerOrRange{ location, found }) => {
                Self::format_line( format!(
                        "the index operator `[]` expected an integer or range as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDotFirstOperandExpectedPlaceOrEvaluable{ location, found }) |
            Self::Semantic(SemanticError::OperatorDotFirstOperandExpectedTuple{ location, found }) |
            Self::Semantic(SemanticError::OperatorDotFirstOperandExpectedInstance { location, found }) => {
                Self::format_line( format!(
                        "the field access operator `.` expected a tuple or object instance as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorDotSecondOperandExpectedIdentifier { location, found }) => {
                Self::format_line( format!(
                        "the field access operator `.` expected a tuple or object instance field identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorPathFirstOperandExpectedPath{ location, found }) => {
                Self::format_line( format!(
                        "the path resolution operator `::` expected an item identifier as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorPathSecondOperandExpectedIdentifier { location, found }) => {
                Self::format_line( format!(
                        "the path resolution operator `::` expected an item identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorStructureFirstOperandExpectedType{ location, found }) => {
                Self::format_line( format!(
                    "the path must point to a structure type, found `{}`",
                    found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::OperatorStructureSecondOperandExpectedLiteral { location, found }) => {
                Self::format_line( format!(
                    "the structure type expected a structure literal, found `{}`",
                    found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ArrayPushingInvalidType { location, expected, found }) => {
                Self::format_line( format!(
                        "expected `{}`, found `{}`",
                        expected, found,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ArrayIndexOutOfRange { location, index, size }) => {
                Self::format_line( format!(
                        "index `{}` is out of range of the array of size {}",
                        index, size,
                    )
                        .as_str(),
                    code, location,
                                   Some("array index must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::ArraySliceStartOutOfRange { location, start }) => {
                Self::format_line( format!(
                        "left slice bound `{}` is negative",
                        start,
                    )
                        .as_str(),
                    code, location,
                                   Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::ArraySliceEndOutOfRange { location, end, size }) => {
                Self::format_line( format!(
                        "right slice bound `{}` is out of range of the array of size {}",
                        end, size,
                    )
                        .as_str(),
                    code, location,
                                   Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::ArraySliceEndLesserThanStart { location, start, end }) => {
                Self::format_line( format!(
                        "left slice bound `{}` is greater than right slice bound `{}`",
                        start, end,
                    )
                        .as_str(),
                    code, location,
                                   Some("left slice range bound must be lesser or equal to the right one"),
                )
            }

            Self::Semantic(SemanticError::TupleFieldOutOfRange { location, r#type, field_index }) => {
                Self::format_line( format!(
                    "`{}` has no field with index `{}`",
                    r#type, field_index,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }

            Self::Semantic(SemanticError::StructureNotInitialized { location, r#type }) => {
                Self::format_line( format!(
                    "`{}` must be initialized with a structure literal",
                    r#type,
                )
                                       .as_str(),
                                   code, location,
                                   Some(format!("consider initializing the value, e.g. `{} {{ a: 42, b: 25, ... }}`", r#type).as_str()),
                )
            }
            Self::Semantic(SemanticError::StructureFieldDoesNotExist { location, r#type, field_name }) => {
                Self::format_line( format!(
                        "field or method `{}` does not exist in `{}`",
                        field_name, r#type,
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::StructureFieldExpected { location, r#type, position, expected, found }) => {
                Self::format_line( format!(
                    "`{}` expected field `{}` at position {}, found `{}`",
                    r#type, expected, position, found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::StructureFieldInvalidType { location, r#type, field_name, expected, found }) => {
                Self::format_line( format!(
                    "field `{}` of `{}` expected type `{}`, found `{}`",
                    field_name, r#type, expected, found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::StructureFieldCount { location, r#type, expected, found }) => {
                Self::format_line( format!(
                    "`{}` expected {} fields, found {}",
                    r#type, expected, found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }

            Self::Semantic(SemanticError::MutatingWithDifferentType { location, expected, found }) => {
                Self::format_line( format!("expected `{}`, found `{}`", expected, found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::MutatingImmutableMemory { location, name, reference }) => {
                Self::format_line_with_reference(format!("cannot assign twice to immutable variable `{}`", name).as_str(),
                    code, location,
                    reference,
                    Some(format!("make this variable mutable: `mut {}`", name).as_str()),
                )
            }
            Self::Semantic(SemanticError::MutatingImmutableContractField { location, name }) => {
                Self::format_line(format!("cannot mutate the immutable contract storage field `{}`", name).as_str(),
                                                 code, location,
                                   Some("such fields cannot be changed by the contract logic"),
                )
            }

            Self::Semantic(SemanticError::TypeAliasExpectedType { location, found }) => {
                Self::format_line( format!(
                        "expected type, found `{}`",
                        found
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::TypeInstantiationForbidden { location, found }) => {
                Self::format_line( format!(
                    "type `{}` cannot be instantiated beyond the contract storage",
                    found,
                )
                                       .as_str(),
                                   code, location,
                                   Some("consider removing strings, ranges, functions, and maps from the type declaration"),
                )
            }
            Self::Semantic(SemanticError::TypeDuplicateField { location, r#type, field_name }) => {
                Self::format_line( format!(
                    "`{}` has a duplicate field `{}`",
                    r#type, field_name,
                )
                                       .as_str(),
                                   code, location,
                                   Some("consider giving the field a unique name"),
                )
            }
            Self::Semantic(SemanticError::TypeDuplicateVariantValue { location, r#type, variant_name, variant_value }) => {
                Self::format_line( format!(
                    "`{}` has a duplicate variant `{}` with value `{}`",
                    r#type, variant_name, variant_value,
                )
                                       .as_str(),
                                   code, location,
                                   Some("variants with the same value are temporarily prohibited"),
                )
            }
            Self::Semantic(SemanticError::TypeUnexpectedGenerics { location, r#type }) => {
                Self::format_line( format!(
                    "type `{}` got unexpected generics",
                    r#type
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::TypeExpectedGenerics { location, r#type, expected }) => {
                Self::format_line( format!(
                    "structure `{}` expected {} generic arguments",
                    r#type, expected,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::TypeInvalidGenericsNumber { location, r#type, expected, found }) => {
                Self::format_line( format!(
                    "structure `{}` expected {} generic arguments, found {}",
                    r#type, expected, found,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }

            Self::Semantic(SemanticError::FunctionArgumentCount { location, function, expected, found, reference }) => {
                Self::format_line_with_reference( format!(
                        "function `{}` expected {} arguments, found {}",
                        function, expected, found
                    )
                        .as_str(),
                                                  code, location,
                    reference,
                    None,
                )
            }
            Self::Semantic(SemanticError::FunctionDebugArgumentCount { location, expected, found }) => {
                Self::format_line( format!(
                    "the `dbg!` function expected {} arguments, but got {}",
                    expected, found,
                )
                                       .as_str(),
                                   code, location,
                                   Some("the number of `dbg!` arguments after the format string must be equal to the number of placeholders, e.g. `dbg!(\"{}, {}\", a, b)`"),
                )
            }
            Self::Semantic(SemanticError::FunctionArgumentType { location, function, name, position, expected, found }) => {
                Self::format_line( format!(
                        "function `{}` expected type `{}` as the argument `{}` (#{}), found `{}`",
                        function, expected, name, position, found
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::FunctionArgumentConstantness { location, function, name, position, found }) => {
                Self::format_line( format!(
                        "function `{}` expected a constant as the argument `{}` (#{}), found a non-constant of type `{}`",
                        function, name, position, found
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::FunctionArgumentNotEvaluable { location, function, position, found }) => {
                Self::format_line( format!(
                        "function `{}` expected a value as the argument #{}, found `{}`",
                        function, position, found
                    )
                        .as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::FunctionReturnType { location, function, expected, found, reference }) => {
                Self::format_line_with_reference(format!(
                        "function `{}` must return a value of type `{}`, found `{}`",
                        function, expected, found
                    )
                        .as_str(),
                    code, location,
                                   Some(reference),
                    None,
                )
            }
            Self::Semantic(SemanticError::FunctionNonCallable { location, name }) => {
                Self::format_line( format!(
                        "attempt to call a non-callable item `{}`",
                        name
                    )
                        .as_str(),
                    code, location,
                                   Some("only functions may be called"),
                )
            }
            Self::Semantic(SemanticError::FunctionCallMutableFromImmutable { location, function }) => {
                Self::format_line(format!(
                    "the mutable method `{}` was called with an immutable instance",
                    function,
                )
                                                     .as_str(),
                                                 code, location,
                                   Some("consider making the instance mutable"),
                )
            }
            Self::Semantic(SemanticError::FunctionUnexpectedExclamationMark { location, function }) => {
                Self::format_line( format!(
                        "attempt to call the `{}` function with an unexpected `!` specifier",
                        function
                    )
                        .as_str(),
                    code, location,
                                   Some("only the `dbg!` function requires the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::FunctionExpectedExclamationMark { location, function }) => {
                Self::format_line( format!(
                        "attempt to call the `{}` function without `!` specifier",
                        function
                    )
                        .as_str(),
                    code, location,
                                   Some("the `dbg!` function requires the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::FunctionStdlibArrayTruncatingToBiggerSize { location, from, to }) => {
                Self::format_line( format!(
                        "attempt to truncate an array from size `{}` to bigger size `{}`",
                        from, to,
                    )
                        .as_str(),
                    code, location,
                                   Some("consider truncating the array to a smaller size"),
                )
            }
            Self::Semantic(SemanticError::FunctionStdlibArrayPaddingToLesserSize { location, from, to }) => {
                Self::format_line( format!(
                        "attempt to pad an array from size `{}` to lesser size `{}`",
                        from, to,
                    )
                        .as_str(),
                    code, location,
                                   Some("consider padding the array to a bigger size"),
                )
            }
            Self::Semantic(SemanticError::FunctionStdlibArrayNewLengthInvalid { location, value }) => {
                Self::format_line( format!(
                        "new array length `{}` cannot act as an index",
                        value,
                    )
                        .as_str(),
                    code, location,
                                   Some("array indexes cannot be greater than maximum of `u64`"),
                )
            }

            Self::Semantic(SemanticError::UnitTestCallForbidden { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be called",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::UnitTestBeyondModuleScope { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` must be declared at the module root scope",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::UnitTestPublicForbidden { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be declared as public",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::UnitTestConstantForbidden { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be declared as constant",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::UnitTestCannotHaveArguments { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot accept arguments",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::UnitTestCannotReturnValue { location, function }) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot return a value",
                    function,
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }

            Self::Semantic(SemanticError::ScopeItemUndeclared { location, name }) => {
                Self::format_line( format!(
                    "cannot find item `{}` in this scope",
                    name
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ScopeItemRedeclared { location, name, reference }) => {
                Self::format_line_with_reference(format!(
                    "item `{}` already declared here",
                    name
                )
                                                     .as_str(),
                                                 code, location,
                                                 reference,
                                                 Some("consider giving the latter item another name"),
                )
            }
            Self::Semantic(SemanticError::ScopeExpectedNamespace { location, name }) => {
                Self::format_line( format!(
                    "item `{}` is not a namespace",
                    name
                )
                                       .as_str(),
                                   code, location,
                                   Some("only modules, structures, enumerations, and contracts can contain items within their namespaces"),
                )
            }
            Self::Semantic(SemanticError::ScopeContractRedeclared { location, reference }) => {
                Self::format_line_with_reference("another contract is already declared here",
                                                 code, location,
                                   Some(reference),
                                                 Some("only one contract may be declared in the project"),
                )
            }
            Self::Semantic(SemanticError::ScopeReferenceLoop { location }) => {
                Self::format_line( "reference loop detected",
                                   code, location,
                                   Some("consider removing circular references between the items"),
                )
            }

            Self::Semantic(SemanticError::ExpressionNonConstantElement { location, found }) => {
                Self::format_line( format!("attempt to use a non-constant value `{}` in a constant expression", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ContractStorageFieldWithoutInstance { location, found }) => {
                Self::format_line( format!("attempt to access the contract storage field `{}` without an instance", found).as_str(),
                                   code, location,
                                   Some(format!("consider accessing the field via a contract instance, e.g. `self.{}`", found).as_str()),
                )
            }

            Self::Semantic(SemanticError::ConditionalExpectedBooleanCondition { location, found }) => {
                Self::format_line( format!("expected `bool`, found `{}`", found).as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ConditionalBranchTypesMismatch { location, expected, found, reference }) => {
                Self::format_line_with_reference(format!("if and else branches return incompatible types `{}` and `{}`", expected, found).as_str(),
                                                 code, location,
                                   Some(reference),
                                                 None,
                )
            }

            Self::Semantic(SemanticError::MatchScrutineeInvalidType { location, found }) => {
                Self::format_line( format!("match scrutinee expected a boolean or integer expression, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::MatchNotExhausted { location }) => {
                Self::format_line( "match expression must be exhaustive",
                    code, location,
                                   Some("ensure that all possible cases are being handled, possibly by adding wildcards or more match arms"),
                )
            }
            Self::Semantic(SemanticError::MatchLessThanTwoBranches { location }) => {
                Self::format_line( "match expression must have at least two branches",
                    code, location,
                                   Some("consider adding some branches to make the expression useful"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchUnreachable { location }) => {
                Self::format_line( "match expression branch is unreachable",
                    code, location,
                                   Some("consider removing the branch or moving it above the branch with a wildcard or irrefutable binding"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternPathExpectedConstant { location, found }) => {
                Self::format_line( format!("expected path to a constant, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::MatchBranchPatternInvalidType { location, expected, found, reference }) => {
                Self::format_line_with_reference(format!("expected `{}`, found `{}`", expected, found).as_str(),
                    code, location,
                                   Some(reference),
                    Some("all branch patterns must be compatible with the type of the expression being matched"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchExpressionInvalidType { location, expected, found, reference }) => {
                Self::format_line_with_reference(format!("expected `{}`, found `{}`", expected, found).as_str(),
                    code, location,
                                   Some(reference),
                    Some("all branches must return the type returned by the first branch"),
                )
            }
            Self::Semantic(SemanticError::MatchBranchDuplicate { location, reference }) => {
                Self::format_line_with_reference("match expression contains a duplicate branch pattern",
                    code, location,
                                   Some(reference),
                    Some("each pattern may occur only once"),
                )
            }

            Self::Semantic(SemanticError::ForStatementWhileExpectedBooleanCondition { location, found }) => {
                Self::format_line( format!("expected `bool`, found `{}`", found).as_str(),
                    code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::ForStatementBoundsExpectedConstantRangeExpression { location, found }) => {
                Self::format_line( format!("expected a constant range expression, found `{}`", found).as_str(),
                    code, location,
                                   Some("only constant ranges allowed, e.g. `for i in 0..42 { ... }`"),
                )
            }

            Self::Semantic(SemanticError::ImplStatementExpectedStructureOrEnumeration { location, found }) => {
                Self::format_line( format!(
                    "`impl` expected a type with namespace, found `{}`",
                    found
                )
                                       .as_str(),
                                   code, location,
                                   Some("only structures and enumerations can have an implementation"),
                )
            }

            Self::Semantic(SemanticError::UseStatementExpectedPath { location, found }) => {
                Self::format_line( format!(
                        "`use` expected an item path, but got `{}`",
                        found
                    )
                        .as_str(),
                    code, location,
                                   Some("consider specifying a valid path to an item to import"),
                )
            }

            Self::Semantic(SemanticError::AttributeUnknown { location, found }) => {
                Self::format_line( format!(
                    "attribute `{}` is unknown",
                    found
                )
                                       .as_str(),
                                   code, location,
                                   Some("see the reference to get the list of allowed attributes"),
                )
            }
            Self::Semantic(SemanticError::AttributeEmpty { location, }) => {
                Self::format_line(
                    "attribute is empty",
                                   code, location,
                                   Some("consider adding an attribute element, e.g. `#[test]`"),
                )
            }
            Self::Semantic(SemanticError::AttributeElementsCount { location, name, expected, found }) => {
                Self::format_line(
                    format!("attribute `{}` expected {} elements, found {}", name, expected, found).as_str(),
                    code, location,
                    None,
                )
            }
            Self::Semantic(SemanticError::AttributeExpectedElement { location, name, position, expected, found }) => {
                Self::format_line(
                    format!("attribute `{}` expected element `{}` at position {}, found `{}`", name, expected, position, found).as_str(),
                    code, location,
                    None,
                )
            }
            Self::Semantic(SemanticError::AttributeExpectedIntegerLiteral { location, name }) => {
                Self::format_line(
                    format!("attribute `{}` expected an integer literal", name).as_str(),
                    code, location,
                    None,
                )
            }
            Self::Semantic(SemanticError::AttributeExpectedNested { location, name }) => {
                Self::format_line(
                    format!("attribute `{}` expected a nested element", name).as_str(),
                    code, location,
                    Some(format!("consider passing the required elements, e.g. `{}(value = 42)`", name).as_str()),
                )
            }

            Self::Semantic(SemanticError::BindingTypeRequired { location, identifier }) => {
                Self::format_line( format!(
                    "type is required for binding `{}`",
                    identifier
                )
                                       .as_str(),
                                   code, location,
                                   Some(format!("consider giving the binding a type, e.g. `{}: u8`", identifier).as_str()),
                )
            }
            Self::Semantic(SemanticError::BindingExpectedTuple { location, expected, found }) => {
                Self::format_line( format!(
                    "expected a tuple with {} elements, found `{}`",
                    expected, found
                )
                                       .as_str(),
                                   code,location,
                None,
                )
            }
            Self::Semantic(SemanticError::BindingSelfNotFirstMethodArgument { location, name, position }) => {
                Self::format_line(format!(
                    "expected the `{}` binding to be at the first position, but found at the position #`{}`",
                    name,
                    position,
                )
                                      .as_str(),
                                  code, location,
                                   Some(format!("consider moving the `{}` binding to the first place", name).as_str()),
                )
            }
            Self::Semantic(SemanticError::BindingFunctionArgumentDestructuringUnavailable { location }) => {
                Self::format_line(
                    "tuple function argument destructuring is not implemented yet",
                    code, location,
                                   Some("consider passing the arguments separately for now"),
                )
            }

            Self::Semantic(SemanticError::EntryPointAmbiguous { main, contract }) => {
                Self::format_line_with_reference("the entry file contains both the `main` function and contract definition",
                                                 code, main,
                    Some(contract),
                    Some("consider choosing between the circuit and contract project type"),
                )
            }
            Self::Semantic(SemanticError::EntryPointConstant { location }) => {
                Self::format_line( "the entry point cannot be constant",
                    code, location,
                                   Some("consider removing the `const` modifier"),
                )
            }
            Self::Semantic(SemanticError::FunctionMainBeyondEntry { location }) => {
                Self::format_line( "the `main` function is declared beyond the `main.zn` entry file",
                    code, location,
                                   Some("the `main` function may be declared only in the entry file"),
                )
            }
            Self::Semantic(SemanticError::ContractBeyondEntry { location }) => {
                Self::format_line( "contract is declared beyond the entry file",
                    code, location,
                                   Some("contracts may be declared only once in the entry file"),
                )
            }
            Self::Semantic(SemanticError::ModuleFileNotFound { location, name }) => {
                Self::format_line( format!(
                        "file not found for module `{}`",
                        name
                    )
                        .as_str(),
                    code, location,
                                   Some(format!("create a file called `{}.zn` inside the module directory", name).as_str()),
                )
            }
        }
    }

    ///
    /// Returns the compiler error code if it is available.
    ///
    pub fn code(&self) -> Option<usize> {
        match self {
            Self::Lexical(_) => None,
            Self::Syntax(_) => None,
            Self::Semantic(inner) => Some(inner.code()),
        }
    }

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error has a location, that is, points to a specific place in the source code.
    ///
    fn format_line(
        message: &str,
        code: Option<usize>,
        location: Location,
        help: Option<&str>,
    ) -> String {
        let index = FILE_INDEX
            .inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION);
        let context = index
            .get(&location.file)
            .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .code
            .lines()
            .collect::<Vec<&str>>();
        let line_number_length = location.line.to_string().len();
        let mut strings = Vec::with_capacity(8);
        strings.push(String::new());
        let code = match code {
            Some(code) => format!("error[{:04}]", code),
            None => "error".to_owned(),
        };
        strings.push(format!("{}: {}", code.bright_red(), message.bright_white()));
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

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error has a second location reference, which helps the user to fix the error more easily.
    ///
    fn format_line_with_reference(
        message: &str,
        code: Option<usize>,
        location: Location,
        reference: Option<Location>,
        help: Option<&str>,
    ) -> String {
        let index = FILE_INDEX
            .inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION);
        let context = index
            .get(&location.file)
            .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .code
            .lines()
            .collect::<Vec<&str>>();
        let line_number_length = location.line.to_string().len();
        let mut strings = Vec::with_capacity(11);
        strings.push(String::new());
        let code = match code {
            Some(code) => format!("error[{:04}]", code),
            None => "error".to_owned(),
        };
        strings.push(format!("{}: {}", code.bright_red(), message.bright_white()));
        if let Some(reference) = reference {
            let context = index
                .get(&reference.file)
                .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
                .code
                .lines()
                .collect::<Vec<&str>>();
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

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error has two location bounds, which enclose the erroneous part of the source code.
    ///
    fn format_range(
        message: &'static str,
        code: Option<usize>,
        start: Location,
        end: Location,
        help: Option<&str>,
    ) -> String {
        let index = FILE_INDEX
            .inner
            .read()
            .expect(zinc_const::panic::SYNCHRONIZATION);
        let context = index
            .get(&start.file)
            .expect(zinc_const::panic::VALIDATED_DURING_SOURCE_CODE_MAPPING)
            .code
            .lines()
            .collect::<Vec<&str>>();
        let line_number_length = end.line.to_string().len();
        let mut strings = Vec::with_capacity(8 + end.line - start.line);
        strings.push(String::new());
        let code = match code {
            Some(code) => format!("error[{:04}]", code),
            None => "error".to_owned(),
        };
        strings.push(format!("{}: {}", code.bright_red(), message.bright_white()));
        strings.push(format!(" {} {}", "-->".bright_cyan(), start));
        strings.push(format!(
            "{}{}",
            " ".repeat(line_number_length + 1),
            "|".bright_cyan()
        ));
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

impl From<ParsingError> for Error {
    fn from(error: ParsingError) -> Self {
        match error {
            ParsingError::Lexical(error) => Self::Lexical(error),
            ParsingError::Syntax(error) => Self::Syntax(error),
        }
    }
}

impl From<SemanticError> for Error {
    fn from(error: SemanticError) -> Self {
        Self::Semantic(error)
    }
}
