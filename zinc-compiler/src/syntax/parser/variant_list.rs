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
use crate::syntax::parser::variant::Parser as VariantParser;
use crate::syntax::tree::variant::Variant;

#[derive(Default)]
pub struct Parser {
    variants: Vec<Variant>,
    next: Option<Token>,
}

impl Parser {
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        mut initial: Option<Token>,
    ) -> Result<(Vec<Variant>, Option<Token>), Error> {
        loop {
            match crate::syntax::parser::take_or_next(initial.take(), stream.clone())? {
                token
                @
                Token {
                    lexeme: Lexeme::Identifier(_),
                    ..
                } => {
                    let variant = VariantParser::default().parse(stream.clone(), Some(token))?;
                    self.variants.push(variant);
                }
                token => return Ok((self.variants, Some(token))),
            }

            match crate::syntax::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use crate::syntax::tree::identifier::Identifier;
    use crate::syntax::tree::literal::integer::Literal as IntegerLiteral;
    use crate::syntax::tree::variant::Variant;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Variant>::new(),
            Some(Token::new(Lexeme::Eof, Location::new(1, 1))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
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

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"A = 1, B = 2, C = 3"#;

        let expected = Ok((
            vec![
                Variant::new(
                    Location::new(1, 1),
                    Identifier::new(Location::new(1, 1), "A".to_owned()),
                    IntegerLiteral::new(
                        Location::new(1, 5),
                        lexical::IntegerLiteral::new_decimal("1".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::new(1, 8),
                    Identifier::new(Location::new(1, 8), "B".to_owned()),
                    IntegerLiteral::new(
                        Location::new(1, 12),
                        lexical::IntegerLiteral::new_decimal("2".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::new(1, 15),
                    Identifier::new(Location::new(1, 15), "C".to_owned()),
                    IntegerLiteral::new(
                        Location::new(1, 19),
                        lexical::IntegerLiteral::new_decimal("3".to_owned()),
                    ),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::new(1, 20))),
        ));

        let result = Parser::default().parse(Rc::new(RefCell::new(TokenStream::new(input))), None);

        assert_eq!(result, expected);
    }
}
