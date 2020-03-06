//!
//! The block expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::error::Error as SyntaxError;
use crate::syntax::parser::statement::local_fn::Parser as FunctionLocalStatementParser;
use crate::syntax::tree::expression::block::builder::Builder as BlockExpressionBuilder;
use crate::syntax::tree::expression::block::Expression as BlockExpression;
use crate::syntax::tree::statement::local_fn::Statement as FunctionLocalStatement;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketCurlyLeft,
    StatementOrBracketCurlyRight,
    BracketCurlyRight,
}

impl Default for State {
    fn default() -> Self {
        State::BracketCurlyLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: BlockExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<BlockExpression, Error> {
        loop {
            match self.state {
                State::BracketCurlyLeft => {
                    match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["{"],
                                lexeme,
                                None,
                            )));
                        }
                    }
                }
                State::StatementOrBracketCurlyRight => {
                    match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        token => {
                            let (statement, next, is_unterminated) =
                                FunctionLocalStatementParser::default()
                                    .parse(stream.clone(), Some(token))?;
                            self.next = next;
                            log::debug!("Block statement: {:?}", statement);
                            match statement {
                                FunctionLocalStatement::Expression(expression) => {
                                    if is_unterminated {
                                        self.builder.set_expression(expression);
                                        self.state = State::BracketCurlyRight;
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
                    return match crate::syntax::parser::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            Err(Error::Syntax(SyntaxError::expected_one_of_or_operator(
                                location,
                                vec!["}"],
                                lexeme,
                                None,
                            )))
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Error;
    use super::Parser;
    use crate::lexical;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Symbol;
    use crate::lexical::TokenStream;
    use crate::syntax::error::Error as SyntaxError;
    use crate::syntax::tree::expression::block::Expression as BlockExpression;
    use crate::syntax::tree::expression::element::Element as ExpressionElement;
    use crate::syntax::tree::expression::object::Object as ExpressionObject;
    use crate::syntax::tree::expression::operand::Operand as ExpressionOperand;
    use crate::syntax::tree::expression::operator::Operator as ExpressionOperator;
    use crate::syntax::tree::expression::Expression;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;

    #[test]
    fn ok_empty() {
        let input = r#"{}"#;

        let expected = Ok(BlockExpression::new(Location::new(1, 1), vec![], None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_statements_with_expression() {
        let input = r#"{ 2 + 1 }"#;

        let expected = Ok(BlockExpression::new(
            Location::new(1, 1),
            vec![],
            Some(Expression::new(
                Location::new(1, 3),
                vec![
                    ExpressionElement::new(
                        Location::new(1, 3),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 3),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 7),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 7),
                                lexical::IntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    ),
                    ExpressionElement::new(
                        Location::new(1, 5),
                        ExpressionObject::Operator(ExpressionOperator::Addition),
                    ),
                ],
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn error_expected_bracket_square_right() {
        let input = r#"{ 42 )"#;

        let expected: Result<_, Error> = Err(Error::Syntax(SyntaxError::expected_one_of(
            Location::new(1, 6),
            vec!["}"],
            Lexeme::Symbol(Symbol::ParenthesisRight),
            None,
        )));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
