//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::Error as SyntaxError;
use crate::error::ParsingError;
use crate::parser::statement::local_fn::Parser as FunctionLocalStatementParser;
use crate::tree::expression::block::builder::Builder as BlockExpressionBuilder;
use crate::tree::expression::block::Expression as BlockExpression;
use crate::tree::statement::local_fn::Statement as FunctionLocalStatement;

///
/// The parser state.
///
#[derive(Debug, Clone, Copy)]
pub enum State {
    /// The initial state.
    BracketCurlyLeft,
    /// The `{` has been parsed so far.
    StatementOrBracketCurlyRight,
    /// The `{` with several statements and probably an expression has been parsed so far.
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        Self::BracketCurlyLeft
    }
}

///
/// The block expression parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parser state.
    state: State,
    /// The builder of the parsed value.
    builder: BlockExpressionBuilder,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a block expression.
    ///
    /// '
    /// {
    ///     let a = 42;
    ///     let b = 25;
    ///     a + b
    /// }
    /// '
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(BlockExpression, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match self.state {
                State::BracketCurlyLeft => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        } => {
                            self.builder.set_location_if_unset(location);
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        Token { lexeme, location } => {
                            return Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["{"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::StatementOrBracketCurlyRight => {
                    match crate::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok((self.builder.finish(), self.next.take())),
                        token => {
                            let (statement, next, is_unterminated) =
                                FunctionLocalStatementParser::default()
                                    .parse(stream.clone(), Some(token))?;
                            self.next = next;

                            match statement {
                                FunctionLocalStatement::Expression(expression) => {
                                    if is_unterminated {
                                        let is_last = matches!(self.next.as_ref().unwrap_or(stream.borrow_mut().look_ahead(1)?), Token {
                                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                                            ..
                                        });

                                        if !is_last && expression.can_be_unterminated() {
                                            self.builder.push_statement(
                                                FunctionLocalStatement::Expression(expression),
                                            );
                                        } else {
                                            self.builder.set_expression(expression);
                                            self.state = State::BracketCurlyRight;
                                        }
                                    } else {
                                        self.builder.push_statement(
                                            FunctionLocalStatement::Expression(expression),
                                        );
                                    }
                                }
                                statement => self.builder.push_statement(statement),
                            }
                        }
                    }
                }
                State::BracketCurlyRight => {
                    return match crate::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => Ok((self.builder.finish(), self.next.take())),
                        Token { lexeme, location } => Err(ParsingError::Syntax(
                            SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["}"],
                                lexeme,
                                None,
                            ),
                        )),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Symbol;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::error::Error as SyntaxError;
    use crate::error::ParsingError;
    use crate::tree::expression::block::Expression as BlockExpression;
    use crate::tree::expression::tree::node::operand::Operand as ExpressionOperand;
    use crate::tree::expression::tree::node::operator::Operator as ExpressionOperator;
    use crate::tree::expression::tree::node::Node as ExpressionTreeNode;
    use crate::tree::expression::tree::Tree as ExpressionTree;
    use crate::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_empty() {
        let input = r#"{}"#;

        let expected = Ok((
            BlockExpression::new(Location::test(1, 1), vec![], None),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_expression() {
        let input = r#"{ 2 + 1 }"#;

        let expected = Ok((
            BlockExpression::new(
                Location::test(1, 1),
                vec![],
                Some(ExpressionTree::new_with_leaves(
                    Location::test(1, 5),
                    ExpressionTreeNode::operator(ExpressionOperator::Addition),
                    Some(ExpressionTree::new(
                        Location::test(1, 3),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 3),
                                LexicalIntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    )),
                    Some(ExpressionTree::new(
                        Location::test(1, 7),
                        ExpressionTreeNode::operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::test(1, 7),
                                LexicalIntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    )),
                )),
            ),
            None,
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"{ 42 )"#;

        let expected: Result<_, ParsingError> =
            Err(ParsingError::Syntax(SyntaxError::expected_one_of(
                Location::test(1, 6),
                vec!["}"],
                Lexeme::Symbol(Symbol::ParenthesisRight),
                None,
            )));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
