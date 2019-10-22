//!
//! The require statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::RequireStatement;
use crate::syntax::RequireStatementBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordRequire,
    ParenthesisLeft,
    Expression,
    CommaOrParenthesisRight,
    Tag,
    ParenthesisRight,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordRequire
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: RequireStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>, mut initial: Option<Token>) -> Result<RequireStatement, Error> {
        loop {
            match self.state {
                State::KeywordRequire => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Require),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::ParenthesisLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["require"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ParenthesisLeft => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => self.state = State::Expression,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Expression => {
                    let (expression, next) = ExpressionParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_expression(expression);
                    self.state = State::CommaOrParenthesisRight;
                }
                State::CommaOrParenthesisRight => {
                    match self.next.take().expect("Always contains a value") {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::Tag,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Tag => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Literal(Literal::String(tag)),
                            ..
                        } => {
                            self.builder.set_tag(tag);
                            self.state = State::ParenthesisRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{string}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ParenthesisRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![")"],
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
    use crate::lexical::BooleanLiteral;
    use crate::lexical::Location;
    use crate::lexical::StringLiteral;
    use crate::lexical::TokenStream;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::Literal;
    use crate::syntax::RequireStatement;

    #[test]
    fn ok() {
        let input = r#"require(true);"#;

        let expected = Ok(RequireStatement::new(
            Location::new(1, 1),
            Expression::new(
                Location::new(1, 9),
                vec![ExpressionElement::new(
                    Location::new(1, 9),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 9),
                        lexical::Literal::Boolean(BooleanLiteral::True),
                    ))),
                )],
            ),
            None,
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_with_annotation() {
        let input = r#"require(true, "test");"#;

        let expected = Ok(RequireStatement::new(
            Location::new(1, 1),
            Expression::new(
                Location::new(1, 9),
                vec![ExpressionElement::new(
                    Location::new(1, 9),
                    ExpressionObject::Operand(ExpressionOperand::Literal(Literal::new(
                        Location::new(1, 9),
                        lexical::Literal::Boolean(BooleanLiteral::True),
                    ))),
                )],
            ),
            Some(StringLiteral::new("test".to_owned())),
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));

        assert_eq!(expected, result);
    }
}
