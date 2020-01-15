//!
//! The field list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Field;
use crate::syntax::FieldParser;

#[derive(Default)]
pub struct Parser {
    fields: Vec<Field>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<Field>, Option<Token>), Error> {
        loop {
            match match initial.take() {
                Some(token) => token,
                None => stream.borrow_mut().next()?,
            } {
                token
                @
                Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => {
                    let (field, next) =
                        FieldParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
                    self.fields.push(field);
                }
                token => return Ok((self.fields, Some(token))),
            }

            match match self.next.take() {
                Some(token) => token,
                None => stream.borrow_mut().next()?,
            } {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => return Ok((self.fields, Some(token))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::Parser;
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Field;
    use crate::syntax::Identifier;
    use crate::syntax::Type;
    use crate::syntax::TypeVariant;

    #[test]
    fn ok_single() {
        let input = r#"a: u232"#;

        let expected = Ok((
            vec![Field::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "a".to_owned()),
                Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 8))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"a: u232,"#;

        let expected = Ok((
            vec![Field::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "a".to_owned()),
                Type::new(Location::new(1, 4), TypeVariant::new_integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 9))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Field>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
