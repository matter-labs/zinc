//!
//! The extern fn statement parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BindingPatternListParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::ExternFnStatement;
use crate::syntax::ExternFnStatementBuilder;
use crate::syntax::Identifier;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordExtern,
    KeywordFn,
    Identifier,
    ParenthesisLeft,
    ArgumentBindingList,
    ParenthesisRight,
    ArrowOrSemicolon,
    ReturnType,
    Semicolon,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordExtern
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: ExternFnStatementBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(ExternFnStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordExtern => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Extern),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::KeywordFn;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["extern"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::KeywordFn => {
                    match match initial.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            ..
                        } => self.state = State::Identifier,
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
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
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
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => self.state = State::ArgumentBindingList,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["("],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ArgumentBindingList => {
                    let (argument_bindings, next) =
                        BindingPatternListParser::default().parse(stream.clone(), None)?;
                    self.builder.set_argument_bindings(argument_bindings);
                    self.next = next;
                    self.state = State::ParenthesisRight;
                }
                State::ParenthesisRight => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => self.state = State::ArrowOrSemicolon,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ArrowOrSemicolon => {
                    match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::MinusGreater),
                            ..
                        } => self.state = State::ReturnType,
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => return Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["->", ";"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ReturnType => {
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.builder.set_return_type(r#type);
                    self.next = next;
                    self.state = State::Semicolon;
                }
                State::Semicolon => {
                    return match match self.next.take() {
                        Some(token) => token,
                        None => stream.borrow_mut().next()?,
                    } {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Semicolon),
                            ..
                        } => Ok((self.builder.finish(), None)),
                        Token { lexeme, location } => Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            vec![";"],
                            lexeme,
                        ))),
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
    use crate::lexical::Location;
    use crate::lexical::TokenStream;
    use crate::syntax::BindingPattern;
    use crate::syntax::BindingPatternVariant;
    use crate::syntax::ExternFnStatement;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_returns_unit() {
        let input = r#"extern fn f(a: field);"#;

        let expected = Ok((
            ExternFnStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 11), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 13),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 13),
                        "a".to_owned(),
                    )),
                    Type::new(Location::new(1, 16), TypeVariant::new_field()),
                )],
                Type::new(Location::new(1, 1), TypeVariant::new_unit()),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_returns_type() {
        let input = r#"extern fn f(a: field) -> field;"#;

        let expected = Ok((
            ExternFnStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 11), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 13),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 13),
                        "a".to_owned(),
                    )),
                    Type::new(Location::new(1, 16), TypeVariant::new_field()),
                )],
                Type::new(Location::new(1, 26), TypeVariant::new_field()),
            ),
            None,
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
