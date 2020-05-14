//!
//! The variant list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use crate::error::Error;
use crate::lexical::stream::TokenStream;
use crate::lexical::token::lexeme::symbol::Symbol;
use crate::lexical::token::lexeme::Lexeme;
use crate::lexical::token::Token;
use crate::syntax::parser::variant::Parser as VariantParser;
use crate::syntax::tree::variant::Variant;

#[derive(Default)]
pub struct Parser {
    variants: Vec<Variant>,
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses an enum variant list.
    ///
    /// 'A = 1, B = 2, C = 3'
    ///
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
                    let (variant, next) =
                        VariantParser::default().parse(stream.clone(), Some(token))?;
                    self.next = next;
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
    use super::Parser;
    use crate::lexical::stream::TokenStream;
    use crate::lexical::token::lexeme::literal::integer::Integer as LexicalIntegerLiteral;
    use crate::lexical::token::lexeme::Lexeme;
    use crate::lexical::token::location::Location;
    use crate::lexical::token::Token;
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

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

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
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

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
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::new(1, 7))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

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
                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::new(1, 8),
                    Identifier::new(Location::new(1, 8), "B".to_owned()),
                    IntegerLiteral::new(
                        Location::new(1, 12),
                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::new(1, 15),
                    Identifier::new(Location::new(1, 15), "C".to_owned()),
                    IntegerLiteral::new(
                        Location::new(1, 19),
                        LexicalIntegerLiteral::new_decimal("3".to_owned()),
                    ),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::new(1, 20))),
        ));

        let result = Parser::default().parse(TokenStream::new(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
