//!
//! The fn statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BlockExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FieldParser;
use crate::syntax::FnStatement;
use crate::syntax::FnStatementBuilder;
use crate::syntax::Identifier;
use crate::syntax::TypeParser;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordFn,
    Identifier,
    ParenthesisLeft,
    Field,
    CommaOrParenthesisRight,
    ArrowOrBody,
    ReturnType,
    Body,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordFn
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: FnStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<FnStatement, Error> {
        loop {
            match self.state {
                State::KeywordFn => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["fn"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::ParenthesisLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
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
                        } => self.state = State::Field,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Field => {
                    let field = FieldParser::default().parse(stream.clone(), None)?;
                    self.builder.push_argument(field);
                    self.state = State::CommaOrParenthesisRight;
                }
                State::CommaOrParenthesisRight => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Comma),
                            ..
                        } => self.state = State::Field,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => self.state = State::ArrowOrBody,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ArrowOrBody => {
                    let next = stream.borrow_mut().next()?;
                    match next {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::MinusGreater),
                            ..
                        } => self.state = State::ReturnType,
                        token => {
                            self.next = Some(token);
                            self.state = State::Body;
                        }
                    }
                }
                State::ReturnType => {
                    let r#type = TypeParser::default().parse(stream.clone(), None)?;
                    self.builder.set_return_type(r#type);
                    self.state = State::Body;
                }
                State::Body => {
                    let body =
                        BlockExpressionParser::default().parse(stream.clone(), self.next.take())?;
                    self.builder.set_body(body);
                    return Ok(self.builder.finish());
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BlockExpression;
    use crate::syntax::Field;
    use crate::syntax::FnStatement;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_returns_unit() {
        let input = r#"fn f(a: field) {}"#;

        let expected = Ok(FnStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 4), "f".to_owned()),
            vec![Field::new(
                Location::new(1, 6),
                Identifier::new(Location::new(1, 6), "a".to_owned()),
                Type::new(Location::new(1, 9), TypeVariant::new_field()),
            )],
            Type::new(Location::new(1, 1), TypeVariant::new_unit()),
            BlockExpression::new(Location::new(1, 16), vec![], None),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_returns_type() {
        let input = r#"fn f(a: field) -> field {}"#;

        let expected = Ok(FnStatement::new(
            Location::new(1, 1),
            Identifier::new(Location::new(1, 4), "f".to_owned()),
            vec![Field::new(
                Location::new(1, 6),
                Identifier::new(Location::new(1, 6), "a".to_owned()),
                Type::new(Location::new(1, 9), TypeVariant::new_field()),
            )],
            Type::new(Location::new(1, 19), TypeVariant::new_field()),
            BlockExpression::new(Location::new(1, 25), vec![], None),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
