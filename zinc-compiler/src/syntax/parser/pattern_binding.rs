//!
//! The binding pattern parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Keyword;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::BindingPattern;
use crate::syntax::BindingPatternBuilder;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::TypeParser;

#[derive(Debug, Clone, Copy)]
pub enum State {
    KeywordMutOrIdentifierOrWildcard,
    IdentifierOrWildcard,
    Colon,
    Type,
}

impl Default for State {
    fn default() -> Self {
        State::KeywordMutOrIdentifierOrWildcard
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: BindingPatternBuilder,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(BindingPattern, Option<Token>), Error> {
        loop {
            match self.state {
                State::KeywordMutOrIdentifierOrWildcard => {
                    match crate::syntax::take_or_next(initial.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Keyword(Keyword::Mut),
                            location,
                        } => {
                            self.builder.set_location(location);
                            self.builder.set_mutable();
                            self.state = State::IdentifierOrWildcard;
                        }
                        token => {
                            self.builder.set_location(token.location);
                            self.next = Some(token);
                            self.state = State::IdentifierOrWildcard;
                        }
                    }
                }
                State::IdentifierOrWildcard => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        } => {
                            self.builder
                                .set_binding(Identifier::new(location, identifier.name));
                            self.state = State::Colon;
                        }
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Underscore),
                            ..
                        } => {
                            self.builder.set_wildcard();
                            self.state = State::Colon;
                        }
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}", "_"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::Colon => {
                    match crate::syntax::take_or_next(self.next.take(), stream.clone())? {
                        Token {
                            lexeme: Lexeme::Symbol(Symbol::Colon),
                            ..
                        } => self.state = State::Type,
                        Token { lexeme, location } => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec![":"],
                                lexeme,
                            )))
                        }
                    }
                }
                State::Type => {
                    let (r#type, next) = TypeParser::default().parse(stream, self.next.take())?;
                    self.builder.set_type(r#type);
                    return Ok((self.builder.finish(), next));
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
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_binding() {
        let input = "value: u8";

        let expected = Ok((
            BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::Binding(Identifier::new(
                    Location::new(1, 1),
                    "value".to_owned(),
                )),
                Type::new(Location::new(1, 8), TypeVariant::integer_unsigned(8)),
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
    fn ok_mutable_binding() {
        let input = "mut value: u8";

        let expected = Ok((
            BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::MutableBinding(Identifier::new(
                    Location::new(1, 5),
                    "value".to_owned(),
                )),
                Type::new(Location::new(1, 12), TypeVariant::integer_unsigned(8)),
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
    fn ok_wildcard() {
        let input = "_: u8";

        let expected = Ok((
            BindingPattern::new(
                Location::new(1, 1),
                BindingPatternVariant::Wildcard,
                Type::new(Location::new(1, 4), TypeVariant::integer_unsigned(8)),
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
