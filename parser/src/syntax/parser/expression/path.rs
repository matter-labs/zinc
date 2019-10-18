//!
//! The path expression parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Identifier;
use crate::syntax::PathExpression;
use crate::syntax::PathExpressionBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Identifier,
    DoubleColonOrEnd,
}

impl Default for State {
    fn default() -> Self {
        State::Identifier
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: PathExpressionBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<PathExpression, Error> {
        loop {
            match self.state {
                State::Identifier => {
                    let next = stream.borrow_mut().next();
                    match next {
                        Some(Ok(Token {
                            lexeme: Lexeme::Identifier(identifier),
                            location,
                        })) => {
                            self.builder.set_location_if_unset(location);
                            self.builder
                                .push_identifier(Identifier::new(location, identifier.name));
                            self.state = State::DoubleColonOrEnd;
                        }
                        Some(Ok(Token { lexeme, location })) => {
                            return Err(Error::Syntax(SyntaxError::Expected(
                                location,
                                vec!["{identifier}"],
                                lexeme,
                            )));
                        }
                        Some(Err(error)) => return Err(Error::Lexical(error)),
                        None => {
                            return Err(Error::Syntax(SyntaxError::UnexpectedEnd(
                                stream.borrow().location(),
                            )))
                        }
                    }
                }
                State::DoubleColonOrEnd => {
                    let peek = stream.borrow_mut().peek();
                    match peek {
                        Some(Ok(Token {
                            lexeme: Lexeme::Symbol(Symbol::DoubleColon),
                            ..
                        })) => {
                            stream.borrow_mut().next();
                            self.state = State::Identifier;
                        }
                        _ => return Ok(self.builder.finish()),
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
    use crate::syntax::Identifier;
    use crate::syntax::PathExpression;

    #[test]
    fn ok() {
        let input = r#"mega::ultra::namespace;"#;

        let expected = Ok(PathExpression::new(
            Location::new(1, 1),
            vec![
                Identifier::new(Location::new(1, 1), "mega".to_owned()),
                Identifier::new(Location::new(1, 7), "ultra".to_owned()),
                Identifier::new(Location::new(1, 14), "namespace".to_owned()),
            ],
        ));

        let result =
            Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input.to_owned()))));;

        assert_eq!(expected, result);
    }
}
