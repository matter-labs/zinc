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
use crate::syntax::BlockExpression;
use crate::syntax::BlockExpressionBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FunctionLocalStatement;
use crate::syntax::FunctionLocalStatementParser;

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
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::StatementOrBracketCurlyRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::StatementOrBracketCurlyRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
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
                    match crate::syntax::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketCurlyRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["}"],
                                lexeme,
                            )));
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

    use super::Parser;
    use crate::lexical;
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::ExpressionOperator;
    use crate::syntax::IntegerLiteral;

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

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#"{}"#;

        let expected = Ok(BlockExpression::new(Location::new(1, 1), vec![], None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(expected, result);
    }
}
