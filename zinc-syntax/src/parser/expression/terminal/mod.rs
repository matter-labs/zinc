//!
//! The terminal operand parser.
//!

pub mod array;
pub mod block;
pub mod conditional;
pub mod list;
pub mod r#match;
pub mod tuple;

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Keyword;
use zinc_lexical::Lexeme;
use zinc_lexical::Literal as LexicalLiteral;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
use crate::tree::expression::tree::Tree as ExpressionTree;
use crate::tree::identifier::builder::Builder as IdentifierBuilder;
use crate::tree::literal::boolean::Literal as BooleanLiteral;
use crate::tree::literal::integer::Literal as IntegerLiteral;
use crate::tree::literal::string::Literal as StringLiteral;

use self::array::Parser as ArrayExpressionParser;
use self::block::Parser as BlockExpressionParser;
use self::conditional::Parser as ConditionalExpressionParser;
use self::r#match::Parser as MatchExpressionParser;
use self::tuple::Parser as TupleExpressionParser;

///
/// The terminal operand parser.
///
#[derive(Default)]
pub struct Parser {
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a lowest-level terminal operand:
    /// - parenthesized expression
    /// - block
    /// - array
    /// - conditional
    /// - match
    /// - alias (`crate`, `super`, `Self`, `self`)
    /// - identifier
    /// - literal (boolean, integer, string)
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(ExpressionTree, Option<Token>), ParsingError> {
        self.next = initial;

        let (operand, location, next) =
            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                    ..
                } => {
                    return TupleExpressionParser::default()
                        .parse(stream, Some(token))
                        .map(|(operand, token)| (operand, token));
                }
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                    ..
                } => {
                    let location = token.location;
                    BlockExpressionParser::default()
                        .parse(stream, Some(token))
                        .map(|(operand, token)| {
                            (ExpressionOperand::Block(operand), location, token)
                        })
                }
                token
                @
                Token {
                    lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                    ..
                } => {
                    let location = token.location;
                    ArrayExpressionParser::default()
                        .parse(stream, Some(token))
                        .map(|(operand, token)| {
                            (ExpressionOperand::Array(operand), location, token)
                        })
                }
                token
                @
                Token {
                    lexeme: Lexeme::Keyword(Keyword::If),
                    ..
                } => {
                    let location = token.location;
                    ConditionalExpressionParser::default()
                        .parse(stream, Some(token))
                        .map(|(operand, token)| {
                            (ExpressionOperand::Conditional(operand), location, token)
                        })
                }
                token
                @
                Token {
                    lexeme: Lexeme::Keyword(Keyword::Match),
                    ..
                } => {
                    let location = token.location;
                    MatchExpressionParser::default()
                        .parse(stream, Some(token))
                        .map(|(operand, token)| {
                            (ExpressionOperand::Match(operand), location, token)
                        })
                }
                Token {
                    lexeme: Lexeme::Keyword(keyword @ Keyword::Crate),
                    location,
                }
                | Token {
                    lexeme: Lexeme::Keyword(keyword @ Keyword::Super),
                    location,
                }
                | Token {
                    lexeme: Lexeme::Keyword(keyword @ Keyword::SelfLowercase),
                    location,
                }
                | Token {
                    lexeme: Lexeme::Keyword(keyword @ Keyword::SelfUppercase),
                    location,
                } => {
                    let mut builder = IdentifierBuilder::default();
                    builder.set_location(location);
                    builder.set_name(keyword.to_string());
                    Ok((
                        ExpressionOperand::Identifier(builder.finish()),
                        location,
                        None,
                    ))
                }
                Token {
                    lexeme: Lexeme::Identifier(identifier),
                    location,
                } => {
                    let mut builder = IdentifierBuilder::default();
                    builder.set_location(location);
                    builder.set_name(identifier.inner);
                    Ok((
                        ExpressionOperand::Identifier(builder.finish()),
                        location,
                        None,
                    ))
                }
                Token {
                    lexeme: Lexeme::Literal(LexicalLiteral::Boolean(boolean)),
                    location,
                } => Ok((
                    ExpressionOperand::LiteralBoolean(BooleanLiteral::new(location, boolean)),
                    location,
                    None,
                )),
                Token {
                    lexeme: Lexeme::Literal(LexicalLiteral::Integer(integer)),
                    location,
                } => Ok((
                    ExpressionOperand::LiteralInteger(IntegerLiteral::new(location, integer)),
                    location,
                    None,
                )),
                Token {
                    lexeme: Lexeme::Literal(LexicalLiteral::String(string)),
                    location,
                } => Ok((
                    ExpressionOperand::LiteralString(StringLiteral::new(location, string)),
                    location,
                    None,
                )),
                Token { lexeme, location } => Err(ParsingError::Syntax(
                    SyntaxError::expected_expression_or_operand(location, lexeme),
                )),
            }?;

        Ok((
            ExpressionTree::new(location, ExpressionTreeNode::Operand(operand)),
            next,
        ))
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::BooleanLiteral as LexicalBooleanLiteral;
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::StringLiteral as LexicalStringLiteral;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::boolean::Literal as BooleanLiteral;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::literal::string::Literal as StringLiteral;

    #[test]
    fn ok_literal_boolean() {
        let input = r#"true"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::test(1, 1),
                ExpressionTreeNode::Operand(ExpressionOperand::LiteralBoolean(
                    BooleanLiteral::new(Location::test(1, 1), LexicalBooleanLiteral::r#true()),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_integer() {
        let input = r#"42"#;

        let expected = Ok((
            ExpressionTree::new(
                Location::test(1, 1),
                ExpressionTreeNode::Operand(ExpressionOperand::LiteralInteger(
                    IntegerLiteral::new(
                        Location::test(1, 1),
                        LexicalIntegerLiteral::new_decimal("42".to_owned()),
                    ),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_literal_string() {
        let input = r#""description""#;

        let expected = Ok((
            ExpressionTree::new(
                Location::test(1, 1),
                ExpressionTreeNode::Operand(ExpressionOperand::LiteralString(StringLiteral::new(
                    Location::test(1, 1),
                    LexicalStringLiteral::new("description".to_owned()),
                ))),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_parenthesized() {
        let input = r#"(2 + 2)"#;

        let expected = Ok((
            ExpressionTree::new_with_leaves(
                Location::test(1, 4),
                ExpressionTreeNode::operator(ExpressionOperator::Addition),
                Some(ExpressionTree::new(
                    Location::test(1, 2),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 2),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    )),
                )),
                Some(ExpressionTree::new(
                    Location::test(1, 6),
                    ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::test(1, 6),
                            LexicalIntegerLiteral::new_decimal("2".to_owned()),
                        ),
                    )),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected() {
        let input = r#"*"#;

        let expected: Result<_, ParsingError> = Err(ParsingError::Syntax(
            SyntaxError::expected_expression_or_operand(
                Location::test(1, 1),
                Lexeme::Symbol(Symbol::Asterisk),
            ),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
