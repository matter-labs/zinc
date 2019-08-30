//!
//! The type parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::lexical::Lexeme;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Error as SyntaxError;
use crate::syntax::Type;
use crate::syntax::TypeBuilder;
use crate::Error;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Name,
    End,
}

impl Default for State {
    fn default() -> Self {
        State::Name
    }
}

#[derive(Default)]
pub struct Parser {
    state: State,
    builder: TypeBuilder,
}

impl Parser {
    pub fn parse(mut self, stream: Rc<RefCell<TokenStream>>) -> Result<Type, Error> {
        loop {
            match self.state {
                State::Name => match stream.borrow_mut().next() {
                    Some(Ok(Token {
                        lexeme: Lexeme::Keyword(keyword),
                        location,
                    })) => {
                        self.builder.set_location(location);
                        self.builder.set_keyword(keyword);
                        self.state = State::End;
                    }
                    Some(Ok(Token { lexeme, location })) => {
                        return Err(Error::Syntax(SyntaxError::Expected(
                            location,
                            ["{type}"].to_vec(),
                            lexeme,
                        )))
                    }
                    Some(Err(error)) => return Err(Error::Lexical(error)),
                    None => return Err(Error::Syntax(SyntaxError::UnexpectedEnd)),
                },
                State::End => return Ok(self.builder.finish()),
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
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok() {
        let code = b"uint228";

        let expected = Type::new(Location::new(1, 1), TypeVariant::uint(228));

        let result = Parser::default()
            .parse(Rc::new(RefCell::new(TokenStream::new(code.to_vec()))))
            .expect("Syntax error");

        assert_eq!(expected, result);
    }
}
