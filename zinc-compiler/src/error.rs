//!
//! The Zinc compiler error.
//!

use colored::Colorize;

use zinc_lexical::Error as LexicalError;
use zinc_lexical::Keyword;
use zinc_lexical::Location;
use zinc_syntax::Error as SyntaxError;
use zinc_syntax::ParsingError;
use zinc_utils::InferenceError;
use zinc_utils::FILE_INDEX;

use crate::semantic::analyzer::attribute::error::Error as AttributeError;
use crate::semantic::analyzer::expression::conditional::error::Error as ConditionalExpressionError;
use crate::semantic::analyzer::expression::error::Error as ExpressionError;
use crate::semantic::analyzer::expression::r#match::error::Error as MatchExpressionError;
use crate::semantic::analyzer::statement::error::Error as StatementError;
use crate::semantic::analyzer::statement::r#for::error::Error as ForStatementError;
use crate::semantic::analyzer::statement::r#impl::error::Error as ImplStatementError;
use crate::semantic::analyzer::statement::r#use::error::Error as UseStatementError;
use crate::semantic::casting::error::Error as CastingError;
use crate::semantic::element::constant::array::error::Error as ArrayConstantError;
use crate::semantic::element::constant::error::Error as ConstantError;
use crate::semantic::element::constant::integer::error::Error as IntegerConstantError;
use crate::semantic::element::constant::structure::error::Error as StructureConstantError;
use crate::semantic::element::constant::tuple::error::Error as TupleConstantError;
use crate::semantic::element::error::Error as ElementError;
use crate::semantic::element::place::error::Error as PlaceError;
use crate::semantic::element::r#type::contract::error::Error as ContractTypeError;
use crate::semantic::element::r#type::enumeration::error::Error as EnumerationTypeError;
use crate::semantic::element::r#type::error::Error as TypeError;
use crate::semantic::element::r#type::function::error::Error as FunctionError;
use crate::semantic::element::r#type::function::intrinsic::debug::error::Error as DebugFunctionError;
use crate::semantic::element::r#type::function::intrinsic::error::Error as IntrinsicFunctionError;
use crate::semantic::element::r#type::function::intrinsic::stdlib::error::Error as StandardLibraryFunctionError;
use crate::semantic::element::r#type::function::test::error::Error as TestFunctionError;
use crate::semantic::element::r#type::structure::error::Error as StructureTypeError;
use crate::semantic::element::value::array::error::Error as ArrayValueError;
use crate::semantic::element::value::contract::error::Error as ContractValueError;
use crate::semantic::element::value::error::Error as ValueError;
use crate::semantic::element::value::integer::error::Error as IntegerValueError;
use crate::semantic::element::value::structure::error::Error as StructureValueError;
use crate::semantic::element::value::tuple::error::Error as TupleValueError;
use crate::semantic::error::Error as SemanticError;
use crate::semantic::scope::error::Error as ScopeError;

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
        match self {
            Self::Lexical(LexicalError::UnterminatedBlockComment { start, end }) => {
                Self::format_range("unterminated block comment", start, end, None)
            }
            Self::Lexical(LexicalError::UnterminatedDoubleQuoteString { start, end }) => {
                Self::format_range(
                    "unterminated double quote string",
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
                location,
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
                location,
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
                location,
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
                location,
                None,
            ),
            Self::Lexical(LexicalError::InvalidCharacter { location, found }) => Self::format_line( format!("invalid character `{}`", found).as_str(),
                location,
                None,
            ),
            Self::Lexical(LexicalError::UnexpectedEnd { location }) => {
                Self::format_line( "unexpected end of input", location, None)
            }
            Self::Syntax(SyntaxError::ExpectedOneOf {
                location,
                expected,
                found,
                help,
            }) => Self::format_line( format!("expected one of {}, found `{}`", expected, found).as_str(),
                location,
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
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedMutOrIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected keyword `mut` or identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedFieldIdentifier {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected field identifier, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedType {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected type, found `{}`", found).as_str(),
                location,
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
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedValue {
                location,
                found,
                help,
            }) => Self::format_line( format!("expected `=` with value, found `{}`", found).as_str(),
                location,
                help,
            ),
            Self::Syntax(SyntaxError::ExpectedExpressionOrOperand { location, found }) => {
                Self::format_line( format!("expected expression or operand, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedIntegerLiteral { location, found }) => {
                Self::format_line( format!("expected integer literal, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedBindingPattern { location, found }) => {
                Self::format_line( format!("expected identifier or `_`, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Syntax(SyntaxError::ExpectedMatchPattern { location, found }) => {
                Self::format_line( format!(
                        "expected identifier, boolean or integer literal, path, or `_`, found `{}`",
                        found
                    )
                    .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseOrFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise OR operator `|=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseOrSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise OR operator `|=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseXorFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise XOR operator `^=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseXorSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise XOR operator `^=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseAndFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise AND operator `&=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseAndSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise AND operator `&=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseShiftLeftFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise shift left operator `<<=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseShiftLeftSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise shift left operator `<<=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseShiftRightFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise shift right operator `>>=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentBitwiseShiftRightSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment bitwise shift right operator `>>=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentAdditionFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `+=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentAdditionSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `+=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentSubtractionFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `-=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentSubtractionSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `-=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentMultiplicationFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `*=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentMultiplicationSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `*=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentDivisionFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `/=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentDivisionSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `/=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentRemainderFirstOperandExpectedPlace{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `%=` expected a memory place as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAssignmentRemainderSecondOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the assignment operator `%=` expected a value as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRangeInclusiveFirstOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRangeInclusiveFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the inclusive range operator `..=` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRangeInclusiveSecondOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRangeInclusiveSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the inclusive range operator `..=` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRangeFirstOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRangeFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the range operator `..` expected an integer constant as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRangeSecondOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRangeSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the range operator `..` expected an integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorOrFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorOrFirstOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorOrFirstOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the OR operator `||` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorOrSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorOrSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorOrSecondOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the OR operator `||` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorXorFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorXorFirstOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorXorFirstOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the XOR operator `^^` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorXorSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorXorSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorXorSecondOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the XOR operator `^^` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAndFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorAndFirstOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorAndFirstOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the AND operator `&&` expected a boolean as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAndSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorAndSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorAndSecondOperandExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the AND operator `&&` expected a boolean as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorEqualsFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorEqualsFirstOperandExpectedPrimitiveType{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorEqualsFirstOperandExpectedPrimitiveType{ location, found }))) => {
                Self::format_line( format!(
                        "the equals operator `==` expected a unit, boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorEqualsSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedUnit{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorEqualsSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedUnit{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorEqualsSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the equals operator `==` expected a unit, boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorNotEqualsFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNotEqualsFirstOperandExpectedPrimitiveType{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNotEqualsFirstOperandExpectedPrimitiveType{ location, found }))) => {
                Self::format_line( format!(
                        "the not equals operator `!=` expected a boolean or integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorNotEqualsSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedUnit{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNotEqualsSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedUnit{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNotEqualsSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the not equals operator `!=` expected a boolean or integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorGreaterEqualsFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorGreaterEqualsFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorGreaterEqualsFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the greater equals operator `>=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorGreaterEqualsSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorGreaterEqualsSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorGreaterEqualsSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the greater equals operator `>=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorLesserEqualsFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorLesserEqualsFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorLesserEqualsFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the lesser equals operator `<=` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorLesserEqualsSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorLesserEqualsSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorLesserEqualsSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the lesser equals operator `<=` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorGreaterFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorGreaterFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorGreaterFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the greater operator `>` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorGreaterSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorGreaterSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorGreaterSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the greater operator `>` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorLesserFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorLesserFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorLesserFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the lesser operator `<` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorLesserSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorLesserSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorLesserSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the lesser operator `<` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseOrFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseOrFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseOrFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise OR operator `|` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseOrSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseOrSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseOrSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise OR operator `|` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseXorFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseXorFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseXorFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise XOR operator `^` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseXorSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseXorSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseXorSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise XOR operator `^` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseAndFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseAndFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseAndFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise AND operator `&` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseAndSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseAndSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseAndSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise AND operator `&` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseShiftLeftFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseShiftLeftFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise shift left operator `<<` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseShiftLeftSecondOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned { location, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseShiftLeftSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OperatorBitwiseShiftLeftSecondOperatorExpectedUnsigned { location, found })))) => {
                Self::format_line( format!(
                        "the bitwise shift left operator `<<` expected an unsigned integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseShiftRightFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseShiftRightFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseShiftRightFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise shift right operator `>>` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseShiftRightSecondOperandExpectedConstant{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseShiftRightSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned { location, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseShiftRightSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OperatorBitwiseShiftRightSecondOperatorExpectedUnsigned { location, found })))) => {
                Self::format_line( format!(
                        "the bitwise shift right operator `>>` expected an unsigned integer constant as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAdditionFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorAdditionFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorAdditionFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the addition operator `+` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorAdditionSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorAdditionSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorAdditionSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the addition operator `+` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorSubtractionFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorSubtractionFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorSubtractionFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the subtraction operator `-` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorSubtractionSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorSubtractionSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorSubtractionSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the subtraction operator `-` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorMultiplicationFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorMultiplicationFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorMultiplicationFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the multiplication operator `*` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorMultiplicationSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorMultiplicationSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorMultiplicationSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the multiplication operator `*` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorDivisionFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorDivisionFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorDivisionFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the division operator `/` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorDivisionSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorDivisionSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorDivisionSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the division operator `/` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRemainderFirstOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorRemainderFirstOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRemainderFirstOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the remainder operator `%` expected an integer as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorRemainderSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorRemainderSecondOperandExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorRemainderSecondOperandExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the remainder operator `%` expected an integer as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorCastingFirstOperandExpectedEvaluable{ location, found })) => {
                Self::format_line( format!(
                        "the casting operator `as` expected a value as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorCastingSecondOperandExpectedType{ location, found })) => {
                Self::format_line( format!(
                        "the casting operator `as` expected a type as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Casting { location, inner: CastingError::CastingFromInvalidType { from, to }, reference }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Casting { location, inner: CastingError::CastingToInvalidType { from, to }, reference }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Casting { location, inner: CastingError::CastingFromInvalidType { from, to }, reference }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Casting { location, inner: CastingError::CastingToInvalidType { from, to }, reference }))) => {
                Self::format_line_with_reference(format!(
                        "cannot cast from `{}` to `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    Some("only integer values can be casted to an integer with different bitlength or field element"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorNotExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNotExpectedBoolean{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNotExpectedBoolean{ location, found }))) => {
                Self::format_line( format!(
                        "the NOT operator `!` expected a boolean, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorBitwiseNotExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorBitwiseNotExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorBitwiseNotExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the bitwise NOT operator `~` expected an integer, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorNegationExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorNegationExpectedInteger{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorNegationExpectedInteger{ location, found }))) => {
                Self::format_line( format!(
                        "the negation operator `-` expected an integer, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorIndexFirstOperandExpectedPlaceOrEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::OperatorIndexFirstOperandExpectedArray{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorIndexFirstOperandExpectedArray{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorIndexFirstOperandExpectedArray{ location, found }))) => {
                Self::format_line( format!(
                        "the index operator `[]` expected an array as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorIndexSecondOperandExpectedEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::OperatorIndexSecondOperandExpectedIntegerOrRange{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorIndexSecondOperandExpectedIntegerOrRange{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorIndexSecondOperandExpectedIntegerOrRange{ location, found }))) => {
                Self::format_line( format!(
                        "the index operator `[]` expected an integer or range as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorDotFirstOperandExpectedPlaceOrEvaluable{ location, found })) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::OperatorDotFirstOperandExpectedTuple{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::OperatorDotFirstOperandExpectedInstance { location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorDotFirstOperandExpectedTuple{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::OperatorDotFirstOperandExpectedInstance { location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorDotFirstOperandExpectedTuple{ location, found }))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::OperatorDotFirstOperandExpectedInstance { location, found }))) => {
                Self::format_line( format!(
                        "the field access operator `.` expected a tuple or object instance as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorDotSecondOperandExpectedIdentifier { location, found })) => {
                Self::format_line( format!(
                        "the field access operator `.` expected a tuple or object instance field identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorPathFirstOperandExpectedPath{ location, found })) => {
                Self::format_line( format!(
                        "the path resolution operator `::` expected an item identifier as the first operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorPathSecondOperandExpectedIdentifier { location, found })) => {
                Self::format_line( format!(
                        "the path resolution operator `::` expected an item identifier as the second operand, found `{}`",
                        found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorStructureFirstOperandExpectedType{ location, found })) => {
                Self::format_line( format!(
                    "the path must point to a structure type, found `{}`",
                    found,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::OperatorStructureSecondOperandExpectedLiteral { location, found })) => {
                Self::format_line( format!(
                    "the structure type expected a structure literal, found `{}`",
                    found,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Array(ArrayValueError::PushingInvalidType { location, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Array(ArrayConstantError::PushingInvalidType { location, expected, found })))) => {
                Self::format_line( format!(
                        "expected `{}`, found `{}`",
                        expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Array(ArrayConstantError::IndexOutOfRange { location, index, size })))) => {
                Self::format_line( format!(
                        "index `{}` is out of range of the array of size {}",
                        index, size,
                    )
                        .as_str(),
                    location,
                    Some("array index must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Array(ArrayValueError::SliceStartOutOfRange { location, start })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Array(ArrayConstantError::SliceStartOutOfRange { location, start })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::ArraySliceStartOutOfRange { location, start }))) => {
                Self::format_line( format!(
                        "left slice bound `{}` is negative",
                        start,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Array(ArrayValueError::SliceEndOutOfRange { location, end, size })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Array(ArrayConstantError::SliceEndOutOfRange { location, end, size })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::ArraySliceEndOutOfRange { location, end, size }))) => {
                Self::format_line( format!(
                        "right slice bound `{}` is out of range of the array of size {}",
                        end, size,
                    )
                        .as_str(),
                    location,
                    Some("slice range bounds must be within the array size"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Array(ArrayValueError::SliceEndLesserThanStart { location, start, end })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Array(ArrayConstantError::SliceEndLesserThanStart { location, start, end })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::ArraySliceEndLesserThanStart { location, start, end }))) => {
                Self::format_line( format!(
                        "left slice bound `{}` is greater than right slice bound `{}`",
                        start, end,
                    )
                        .as_str(),
                    location,
                    Some("left slice range bound must be lesser or equal to the right one"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Structure(StructureValueError::NotInitialized { location, type_identifier })))) => {
                Self::format_line( format!(
                    "`{}` must be initialized with a structure literal",
                    type_identifier,
                )
                                       .as_str(),
                                   location,
                                   Some(format!("consider initializing the value, e.g. `{} {{ a: 42, b: 25, ... }}`", type_identifier).as_str()),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Tuple(TupleValueError::FieldOutOrRange { location, type_identifier, field_index })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Tuple(TupleConstantError::FieldOutOrRange { location, type_identifier, field_index })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::TupleFieldOutOfRange { location, type_identifier, field_index }))) => {
                Self::format_line( format!(
                        "`{}` has no field with index `{}`",
                        type_identifier, field_index,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Structure(StructureValueError::FieldDoesNotExist { location, type_identifier, field_name })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Structure(StructureConstantError::FieldDoesNotExist { location, type_identifier, field_name })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::StructureFieldDoesNotExist { location, type_identifier, field_name }))) => {
                Self::format_line( format!(
                        "field or method `{}` does not exist in `{}`",
                        field_name, type_identifier,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Contract(ContractValueError::FieldDoesNotExist { location, type_identifier, field_name })))) |
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::ContractFieldDoesNotExist { location, type_identifier, field_name }))) => {
                Self::format_line( format!(
                        "field or method `{}` does not exist in `{}`",
                        field_name, type_identifier,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::MutatingWithDifferentType { location, expected, found }))) => {
                Self::format_line( format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::MutatingImmutableMemory { location, name, reference }))) => {
                Self::format_line_with_reference(format!("cannot assign twice to immutable variable `{}`", name).as_str(),
                    location,
                    reference,
                    Some(format!("make this variable mutable: `mut {}`", name).as_str()),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Place(PlaceError::MutatingImmutableContractField { location, name }))) => {
                Self::format_line(format!("cannot mutate the immutable contract storage field `{}`", name).as_str(),
                                                 location,
                                                 Some("such fields cannot be changed by the contract logic"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Structure(StructureValueError::FieldExpected { location, type_identifier, position, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Contract(ContractValueError::FieldExpected { location, type_identifier, position, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Structure(StructureConstantError::FieldExpected { location, type_identifier, position, expected, found })))) => {
                Self::format_line( format!(
                        "`{}` expected field `{}` at position {}, found `{}`",
                        type_identifier, expected, position, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Structure(StructureValueError::FieldInvalidType { location, type_identifier, field_name, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Contract(ContractValueError::FieldInvalidType { location, type_identifier, field_name, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Structure(StructureConstantError::FieldInvalidType { location, type_identifier, field_name, expected, found })))) => {
                Self::format_line( format!(
                        "field `{}` of `{}` expected type `{}`, found `{}`",
                        field_name, type_identifier, expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Structure(StructureValueError::FieldOutOfRange { location, type_identifier, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Contract(ContractValueError::FieldOutOfRange { location, type_identifier, expected, found })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Structure(StructureConstantError::FieldOutOfRange { location, type_identifier, expected, found })))) => {
                Self::format_line( format!(
                        "`{}` expected {} fields, found {}",
                        type_identifier, expected, found,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchEquals{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchEquals{ location, first, second })))) => {
                Self::format_line( format!(
                        "the equals operator `==` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchNotEquals{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchNotEquals{ location, first, second })))) => {
                Self::format_line( format!(
                        "the not equals operator `!=` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchGreaterEquals{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchGreaterEquals{ location, first, second })))) => {
                Self::format_line( format!(
                        "the greater equals operator `>=` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchLesserEquals{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchLesserEquals{ location, first, second })))) => {
                Self::format_line( format!(
                        "the lesser equals operator `<=` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchGreater{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchGreater{ location, first, second })))) => {
                Self::format_line( format!(
                        "the greater operator `>` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchLesser{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchLesser{ location, first, second })))) => {
                Self::format_line( format!(
                        "the lesser operator `<` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchBitwiseOr{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchBitwiseOr{ location, first, second })))) => {
                Self::format_line( format!(
                        "the bitwise OR operator `|` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchBitwiseXor{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchBitwiseXor{ location, first, second })))) => {
                Self::format_line( format!(
                        "the bitwise XOR operator `^` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchBitwiseAnd{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchBitwiseAnd{ location, first, second })))) => {
                Self::format_line( format!(
                        "the bitwise AND operator `&` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchAddition{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchAddition{ location, first, second })))) => {
                Self::format_line( format!(
                        "the addition operator `+` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchSubtraction{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchSubtraction{ location, first, second })))) => {
                Self::format_line( format!(
                        "the subtraction operator `-` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchMultiplication{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchMultiplication{ location, first, second })))) => {
                Self::format_line( format!(
                        "the multiplication operator `*` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchDivision{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchDivision{ location, first, second })))) => {
                Self::format_line( format!(
                        "the division operator `/` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::TypesMismatchRemainder{ location, first, second })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::TypesMismatchRemainder{ location, first, second })))) => {
                Self::format_line( format!(
                        "the remainder operator `%` expected two integers of the same type, found `{}` and `{}`",
                        first, second,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowAddition { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the addition operator `+` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowSubtraction { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the subtraction operator `-` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowMultiplication { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the multiplication operator `*` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowDivision { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the division operator `/` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowRemainder { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the remainder operator `%` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowCasting { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the casting operator `as` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::OverflowNegation { location, value, r#type })))) => {
                Self::format_line( format!(
                        "the negation operator `-` overflow, as the value `{}` cannot be represeneted by type `{}`",
                        value, r#type,
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::ForbiddenFieldDivision { location })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ForbiddenFieldDivision { location })))) => {
                Self::format_line( "the division operator `/` is forbidden for the `field` type",
                    location,
                    Some("for inversion consider using `std::ff::invert`"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::ForbiddenFieldRemainder { location })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ForbiddenFieldRemainder { location })))) => {
                Self::format_line( "the remainder operator `%` is forbidden for the `field` type",
                    location,
                    Some("`field` type values cannot be used to get a remainder"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::ForbiddenSignedBitwise { location })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ForbiddenSignedBitwise { location })))) => {
                Self::format_line( "the bitwise operators are forbidden for the signed integer types",
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::ForbiddenFieldBitwise { location })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ForbiddenFieldBitwise { location })))) => {
                Self::format_line( "the bitwise operators are forbidden for the `field` type",
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Value(ValueError::Integer(IntegerValueError::ForbiddenFieldNegation { location })))) |
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ForbiddenFieldNegation { location })))) => {
                Self::format_line( "the negation operator `-` is forbidden for the `field` type",
                    location,
                    Some("`field` type values cannot be negative"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroDivision { location })))) => {
                Self::format_line( "division by zero",
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::ZeroRemainder { location })))) => {
                Self::format_line( "remainder of division by zero",
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::IntegerTooLarge { location, inner: InferenceError::Overflow { value, is_signed, bitlength } })))) => {
                Self::format_line( format!("`{}` is larger than `{}` bits with sign `{}`", value, bitlength, is_signed).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::Parsing { location, inner: zinc_utils::BigIntError::NumberParsing(inner) })))) => {
                Self::format_line(format!("The number parsing error: {}", inner).as_str(),
                                  location,
                                  None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::Parsing { location, inner: zinc_utils::BigIntError::ExponentParsing(inner) })))) => {
                Self::format_line(format!("The exponent value parsing error: {}", inner).as_str(),
                                  location,
                                  None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Constant(ConstantError::Integer(IntegerConstantError::Parsing { location, inner: zinc_utils::BigIntError::ExponentTooSmall(exponent) })))) => {
                Self::format_line(format!("The exponent value `{}` is too small", exponent).as_str(),
                                   location,
                                   Some("The exponent value must be equals or greater than the number of fractional digits"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::AliasDoesNotPointToType { location, found }))) => {
                Self::format_line( format!(
                        "expected type, found `{}`",
                        found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::ItemRedeclared { location, name, reference })) => {
                Self::format_line_with_reference(format!(
                        "item `{}` already declared here",
                        name
                    )
                        .as_str(),
                    location,
                    reference,
                    Some("consider giving the latter item another name"),
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::ItemUndeclared { location, name })) => {
                Self::format_line( format!(
                        "cannot find item `{}` in this scope",
                        name
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::ItemIsNotANamespace { location, name })) => {
                Self::format_line( format!(
                        "item `{}` is not a namespace",
                        name
                    )
                        .as_str(),
                    location,
                    Some("only modules, structures, enumerations, and contracts can contain items within their namespaces"),
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::AssociatedItemWithoutOwner { location, name })) => {
                Self::format_line( format!(
                    "associated item `{}` is accessed without specifying its namespace or entity",
                    name,
                )
                       .as_str(),
                   location,
                   Some("consider adding the namespace or entity prefix to the item"),
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::ContractRedeclared { location, reference })) => {
                Self::format_line_with_reference("another contract is already declared here",
                    location,
                    Some(reference),
                    Some("only one contract may be declared in the project"),
                )
            }
            Self::Semantic(SemanticError::Scope(ScopeError::ReferenceLoop { location })) => {
                Self::format_line( "reference loop detected",
                    location,
                    Some("consider removing circular references between the items"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::ArgumentCount { location, function, expected, found, reference })))) => {
                Self::format_line_with_reference( format!(
                        "function `{}` expected {} arguments, found {}",
                        function, expected, found
                    )
                        .as_str(),
                    location,
                    reference,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::ArgumentType { location, function, name, position, expected, found })))) => {
                Self::format_line( format!(
                        "function `{}` expected type `{}` as the argument `{}` (#{}), found `{}`",
                        function, expected, name, position, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::ArgumentConstantness { location, function, name, position, found })))) => {
                Self::format_line( format!(
                        "function `{}` expected a constant as the argument `{}` (#{}), found a non-constant of type `{}`",
                        function, name, position, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::ArgumentNotEvaluable { location, function, position, found })))) => {
                Self::format_line( format!(
                        "function `{}` expected a value as the argument #{}, found `{}`",
                        function, position, found
                    )
                        .as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::ReturnType { location, function, expected, found, reference })))) => {
                Self::format_line_with_reference(format!(
                        "function `{}` must return a value of type `{}`, found `{}`",
                        function, expected, found
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::NonCallable { location, name })))) => {
                Self::format_line( format!(
                        "attempt to call a non-callable item `{}`",
                        name
                    )
                        .as_str(),
                    location,
                    Some("only functions may be called"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::FunctionMethodSelfNotFirst { location, function, position, reference })))) => {
                Self::format_line_with_reference(format!(
                        "method `{}` expected the `{}` binding to be at the first position, but found at the position #`{}`",
                        function,
                        Keyword::SelfLowercase.to_string(),
                        position,
                    )
                        .as_str(),
                    location,
                    Some(reference),
                    Some(format!("consider moving the `{}` binding to the first place", Keyword::SelfLowercase.to_string()).as_str()),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::CallingMutableFromImmutable { location, function })))) => {
                Self::format_line(format!(
                    "the mutable method `{}` was called with an immutable instance",
                    function,
                )
                                                     .as_str(),
                                                 location,
                                                 Some("consider making the instance mutable"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Unknown { location, function }))))) => {
                Self::format_line( format!(
                        "attempt to call a non-intrinsic function `{}` with `!` specifier",
                        function
                    )
                        .as_str(),
                    location,
                    Some("only intrinsic functions require the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::ExclamationMarkMissing { location, function }))))) => {
                Self::format_line( format!(
                        "attempt to call an intrinsic function `{}` without `!` specifier",
                        function
                    )
                        .as_str(),
                    location,
                    Some("intrinsic functions require the `!` symbol after the function name"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::Debug(DebugFunctionError::ArgumentCount { location, expected, found })))))) => {
                Self::format_line( format!(
                        "the `dbg!` function expected {} arguments, but got {}",
                        expected, found,
                    )
                        .as_str(),
                    location,
                    Some("the number of `dbg!` arguments after the format string must be equal to the number of placeholders, e.g. `dbg!(\"{}, {}\", a, b)`"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayTruncatingToBiggerSize { location, from, to })))))) => {
                Self::format_line( format!(
                        "attempt to truncate an array from size `{}` to bigger size `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("consider truncating the array to a smaller size"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayPaddingToLesserSize { location, from, to })))))) => {
                Self::format_line( format!(
                        "attempt to pad an array from size `{}` to lesser size `{}`",
                        from, to,
                    )
                        .as_str(),
                    location,
                    Some("consider padding the array to a bigger size"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Intrinsic(IntrinsicFunctionError::StandardLibrary(StandardLibraryFunctionError::ArrayNewLengthInvalid { location, value })))))) => {
                Self::format_line( format!(
                        "new array length `{}` cannot act as an index",
                        value,
                    )
                        .as_str(),
                    location,
                    Some("array indexes cannot be greater than maximum of `u64`"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::CallForbidden { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be called",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::BeyondModuleScope { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` must be declared at the module root scope",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::PublicForbidden { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be declared as public",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::ConstantForbidden { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot be declared as constant",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::CannotHaveArguments { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot accept arguments",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Function(FunctionError::Test(TestFunctionError::CannotReturnValue { location, function }))))) => {
                Self::format_line( format!(
                    "unit test function `{}` cannot return a value",
                    function,
                )
                                       .as_str(),
                                   location,
                                   None,
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Structure(StructureTypeError::DuplicateField { location, type_identifier, field_name })))) => {
                Self::format_line( format!(
                        "`{}` has a duplicate field `{}`",
                        type_identifier, field_name,
                    )
                        .as_str(),
                    location,
                    Some("consider giving the field a unique name"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Enumeration(EnumerationTypeError::DuplicateVariantValue { location, type_identifier, variant_name, variant_value })))) => {
                Self::format_line( format!(
                    "`{}` has a duplicate variant `{}` with value `{}`",
                    type_identifier, variant_name, variant_value,
                )
                                       .as_str(),
                                   location,
                                   Some("variants with the same value are temporarily prohibited"),
                )
            }
            Self::Semantic(SemanticError::Element(ElementError::Type(TypeError::Contract(ContractTypeError::DuplicateField { location, type_identifier, field_name })))) => {
                Self::format_line( format!(
                        "`{}` has a duplicate field `{}`",
                        type_identifier, field_name,
                    )
                        .as_str(),
                    location,
                    Some("consider giving the field a unique name"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::NonConstantElement { location, found })) => {
                Self::format_line( format!("attempt to use a non-constant value `{}` in a constant expression", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::ScrutineeInvalidType { location, found }))) => {
                Self::format_line( format!("match scrutinee expected a boolean or integer expression, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::NotExhausted { location }))) => {
                Self::format_line( "match expression must be exhaustive",
                    location,
                    Some("ensure that all possible cases are being handled, possibly by adding wildcards or more match arms"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::LessThanTwoBranches { location }))) => {
                Self::format_line( "match expression must have at least two branches",
                    location,
                    Some("consider adding some branches to make the expression useful"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::BranchUnreachable { location }))) => {
                Self::format_line( "match expression branch is unreachable",
                    location,
                    Some("consider removing the branch or moving it above the branch with a wildcard or irrefutable binding"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::BranchPatternPathExpectedConstant { location, found }))) => {
                Self::format_line( format!("expected path to a constant, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::BranchPatternInvalidType { location, expected, found, reference }))) => {
                Self::format_line_with_reference(format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branch patterns must be compatible with the type of the expression being matched"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::BranchExpressionInvalidType { location, expected, found, reference }))) => {
                Self::format_line_with_reference(format!("expected `{}`, found `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    Some("all branches must return the type returned by the first branch"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Match(MatchExpressionError::BranchDuplicate { location, reference }))) => {
                Self::format_line_with_reference("match expression contains a duplicate branch pattern",
                    location,
                    Some(reference),
                    Some("each pattern may occur only once"),
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Conditional(ConditionalExpressionError::ExpectedBooleanCondition { location, found }))) => {
                Self::format_line( format!("expected `bool`, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Expression(ExpressionError::Conditional(ConditionalExpressionError::BranchTypesMismatch { location, expected, found, reference }))) => {
                Self::format_line_with_reference(format!("if and else branches return incompatible types `{}` and `{}`", expected, found).as_str(),
                    location,
                    Some(reference),
                    None,
                )
            }

            Self::Semantic(SemanticError::Statement(StatementError::For(ForStatementError::WhileExpectedBooleanCondition { location, found }))) => {
                Self::format_line( format!("expected `bool`, found `{}`", found).as_str(),
                    location,
                    None,
                )
            }
            Self::Semantic(SemanticError::Statement(StatementError::For(ForStatementError::BoundsExpectedConstantRangeExpression { location, found }))) => {
                Self::format_line( format!("expected a constant range expression, found `{}`", found).as_str(),
                    location,
                    Some("only constant ranges allowed, e.g. `for i in 0..42 { ... }`"),
                )
            }
            Self::Semantic(SemanticError::Statement(StatementError::Use(UseStatementError::ExpectedPath { location, found }))) => {
                Self::format_line( format!(
                        "`use` expected an item path, but got `{}`",
                        found
                    )
                        .as_str(),
                    location,
                    Some("consider specifying a valid path to an item to import"),
                )
            }
            Self::Semantic(SemanticError::Statement(StatementError::Impl(ImplStatementError::ExpectedStructureOrEnumeration { location, found }))) => {
                Self::format_line( format!(
                        "`impl` expected a type with namespace, found `{}`",
                        found
                    )
                        .as_str(),
                    location,
                    Some("only structures and enumerations can have an implementation"),
                )
            }

            Self::Semantic(SemanticError::Attribute(AttributeError::Unknown { location, found })) => {
                Self::format_line( format!(
                    "unknown attribute `{}`",
                    found
                )
                                       .as_str(),
                                   location,
                                   Some("see the reference to get the list of allowed attributes"),
                )
            }

            Self::Semantic(SemanticError::EntryPointMissing) => {
                Self::format_message(
                    "the project entry point is missing",
                    Some("create the `main` function or a contract definition in the entry point file"),
                )
            }
            Self::Semantic(SemanticError::EntryPointAmbiguous { main, contract }) => {
                Self::format_line_with_reference("the entry file contains both the `main` function and contract definition",
                    main,
                    Some(contract),
                    Some("consider choosing between the circuit and contract project type"),
                )
            }
            Self::Semantic(SemanticError::EntryPointConstant { location }) => {
                Self::format_line( "the entry point cannot be constant",
                    location,
                    Some("consider removing the `const` modifier"),
                )
            }
            Self::Semantic(SemanticError::FunctionMainBeyondEntry { location }) => {
                Self::format_line( "the `main` function is declared beyond the `main.zn` entry file",
                    location,
                    Some("the `main` function may be declared only in the entry file"),
                )
            }
            Self::Semantic(SemanticError::ContractBeyondEntry { location }) => {
                Self::format_line( "contract is declared beyond the entry file",
                    location,
                    Some("contracts may be declared only once in the entry file"),
                )
            }
            Self::Semantic(SemanticError::ModuleFileNotFound { location, name }) => {
                Self::format_line( format!(
                        "file not found for module `{}`",
                        name
                    )
                        .as_str(),
                    location,
                    Some(format!("create a file called `{}.zn` inside the module directory", name).as_str()),
                )
            }
        }
    }

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error is locationless, that is, not related to any specific place in the source code.
    ///
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

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error has a location, that is, points to a specific place in the source code.
    ///
    fn format_line(message: &str, location: Location, help: Option<&str>) -> String {
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

    ///
    /// Formats an error `message` with an optional `help` message.
    ///
    /// The error has a second location reference, which helps the user to fix the error more easily.
    ///
    fn format_line_with_reference(
        message: &str,
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
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
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
        strings.push(format!(
            "{}: {}",
            "error".bright_red(),
            message.bright_white()
        ));
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
