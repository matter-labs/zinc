//!
//! The fn statement parser.
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
use crate::syntax::BlockExpressionParser;
use crate::syntax::Error as SyntaxError;
use crate::syntax::FnStatement;
use crate::syntax::FnStatementBuilder;
use crate::syntax::Identifier;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordFn,
    Identifier,
    ParenthesisLeft,
    ArgumentBindingList,
    ParenthesisRight,
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
    ) -> Result<(FnStatement, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordFn => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Fn),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.state = State::Identifier;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec!["fn"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::Identifier => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            let identifier = Identifier::new(location, identifier.name);
                            self.builder.set_identifier(identifier);
                            self.state = State::ParenthesisLeft;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_identifier(
                                location, lexeme,
                            )));
                        }
                    }
                }
                State::ParenthesisLeft => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisLeft),
                            ..
                        } => self.state = State::ArgumentBindingList,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
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
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::ParenthesisRight),
                            ..
                        } => self.state = State::ArrowOrBody,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::expected_one_of(
                                location,
                                vec![",", ")"],
                                lexeme,
                            )));
                        }
                    }
                }
                State::ArrowOrBody => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
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
                    let (r#type, next) = TypeParser::default().parse(stream.clone(), None)?;
                    self.next = next;
                    self.builder.set_return_type(r#type);
                    self.state = State::Body;
                }
                State::Body => {
                    let body = BlockExpressionParser::default().parse(stream, self.next.take())?;
                    self.builder.set_body(body);
                    return Ok((self.builder.finish(), None));
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
    use crate::syntax::BlockExpression;
    use crate::syntax::FnStatement;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_returns_unit() {
        let input = r#"fn f(a: field) {}"#;

        let expected = Ok((
            FnStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 4), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 6),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 6),
                        "a".to_owned(),
                    )),
                    Type::new(Location::new(1, 9), TypeVariant::field()),
                )],
                Type::new(Location::new(1, 1), TypeVariant::unit()),
                BlockExpression::new(Location::new(1, 16), vec![], None),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_returns_type() {
        let input = r#"fn f(a: field) -> field {}"#;

        let expected = Ok((
            FnStatement::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 4), "f".to_owned()),
                vec![BindingPattern::new(
                    Location::new(1, 6),
                    BindingPatternVariant::Binding(Identifier::new(
                        Location::new(1, 6),
                        "a".to_owned(),
                    )),
                    Type::new(Location::new(1, 9), TypeVariant::field()),
                )],
                Type::new(Location::new(1, 19), TypeVariant::field()),
                BlockExpression::new(Location::new(1, 25), vec![], None),
            ),
            None,
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
