//!
//! The variant list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::Lexeme;
use crate::lexical::Symbol;
use crate::lexical::Token;
use crate::lexical::TokenStream;
use crate::syntax::Variant;
use crate::syntax::VariantParser;

#[derive(Default)]
pub struct Parser {
    variants: Vec<Variant>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<Variant>, Option<Token>), Error> {
        loop {
            match match initial.take() {
                Some(token) => token,
                None => stream.borrow_mut().next()?,
            } {
                token @ Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => {
                    let variant = VariantParser::default().parse(stream.clone(), Some(token))?;
                    self.variants.push(variant);
                }
                token => return Ok((self.variants, Some(token))),
            }

            let next = stream.borrow_mut().next()?;
            match next {
                Token {
                    lexeme: Lexeme::Symbol(Symbol::Comma),
                    ..
                } => continue,
                token => return Ok((self.variants, Some(token))),
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
    use crate::lexical::Lexeme;
    use crate::lexical::Location;
    use crate::lexical::Token;
    use crate::lexical::TokenStream;
    use crate::syntax::Identifier;
    use crate::syntax::IntegerLiteral;
    use crate::syntax::Variant;

    #[test]
    fn ok_single() {
        let input = r#"A = 1"#;

        let expected = Ok((
            vec![Variant::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::new(1, 5),
                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"A = 1,"#;

        let expected = Ok((
            vec![Variant::new(
                Location::new(1, 1),
                Identifier::new(Location::new(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::new(1, 5),
                    lexical::IntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
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
            Vec::<Variant>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(
            Rc::new(RefCell::new(TokenStream::new(input.to_owned()))),
            None,
        );

        assert_eq!(expected, result);
    }
}
