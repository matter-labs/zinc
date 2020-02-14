//!
//! The array expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Literal;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::ArrayExpression;
use crate::syntax::ArrayExpressionBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExpressionParser;
use crate::syntax::IntegerLiteral;

#[derive(Debug, Clone, Copy)]
pub enum State {
    BracketSquareLeft,
    FirstExpressionOrBracketSquareRight,
    ExpressionOrBracketSquareRight,
    CommaOrBracketSquareRight,
    CommaOrSemicolonOrBracketSquareRight,
    SizeLiteral,
    BracketSquareRight,
}

impl Default for State {
    fn default() -> Self {
        State::BracketSquareLeft
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ArrayExpressionBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<ArrayExpression, Error> {
        loop {
            match self.state {
                State::BracketSquareLeft => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareLeft),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::FirstExpressionOrBracketSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["["],
                                lexeme,
                            )));
                        }
                    }
                }
                State::FirstExpressionOrBracketSquareRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        token => {
                            let (expression, next) =
                                ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrSemicolonOrBracketSquareRight;
                        }
                    }
                }
                State::ExpressionOrBracketSquareRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        token => {
                            let (expression, next) =
                                ExpressionParser::default().parse(stream.clone(), Some(token))?;
                            self.next = next;
                            self.builder.push_expression(expression);
                            self.state = State::CommaOrBracketSquareRight;
                        }
                    }
                }
                State::CommaOrBracketSquareRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::ExpressionOrBracketSquareRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", "]"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::CommaOrSemicolonOrBracketSquareRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => {
                            self.state = State::ExpressionOrBracketSquareRight;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => {
                            self.state = State::SizeLiteral;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ";", "]"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::SizeLiteral => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Literal(Literal::Integer(integer)),
                            location,
                        } => {
                            self.builder
                                .set_repeats_count(IntegerLiteral::new(location, integer));
                            self.state = State::BracketSquareRight;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{integer}"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::BracketSquareRight => {
                    match crate::syntax::take_or_next(self.next.take(), stream)? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::BracketSquareRight),
                            ..
                        } => return Ok(self.builder.finish()),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["]"],
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
    use crate::syntax::ArrayExpression;
    use crate::syntax::Expression;
    use crate::syntax::ExpressionElement;
    use crate::syntax::ExpressionObject;
    use crate::syntax::ExpressionOperand;
    use crate::syntax::IntegerLiteral;

    #[test]
    fn ok() {
        let input = r#"[1, 2, 3]"#;

        let expected = Ok(ArrayExpression::new(
            Location::new(1, 1),
            vec![
                Expression::new(
                    Location::new(1, 2),
                    vec![ExpressionElement::new(
                        Location::new(1, 2),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 2),
                                lexical::IntegerLiteral::new_decimal("1".to_owned()),
                            ),
                        )),
                    )],
                ),
                Expression::new(
                    Location::new(1, 5),
                    vec![ExpressionElement::new(
                        Location::new(1, 5),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 5),
                                lexical::IntegerLiteral::new_decimal("2".to_owned()),
                            ),
                        )),
                    )],
                ),
                Expression::new(
                    Location::new(1, 8),
                    vec![ExpressionElement::new(
                        Location::new(1, 8),
                        ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                            IntegerLiteral::new(
                                Location::new(1, 8),
                                lexical::IntegerLiteral::new_decimal("3".to_owned()),
                            ),
                        )),
                    )],
                ),
            ],
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_repeats() {
        let input = r#"[1; 10]"#;

        let expected = Ok(ArrayExpression::new(
            Location::new(1, 1),
            vec![Expression::new(
                Location::new(1, 2),
                vec![ExpressionElement::new(
                    Location::new(1, 2),
                    ExpressionObject::Operand(ExpressionOperand::LiteralInteger(
                        IntegerLiteral::new(
                            Location::new(1, 2),
                            lexical::IntegerLiteral::new_decimal("1".to_owned()),
                        ),
                    )),
                )],
            )],
            Some(IntegerLiteral::new(
                Location::new(1, 5),
                lexical::IntegerLiteral::new_decimal("10".to_owned()),
            )),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_empty() {
        let input = r#"[]"#;

        let expected = Ok(ArrayExpression::new(Location::new(1, 1), vec![], None));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
