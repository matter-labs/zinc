//!
//! The variant list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::variant::Parser as VariantParser;
use crate::tree::variant::Variant;

///
/// The variant list parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parsed variants.
    variants: Vec<Variant>,
    /// The token returned from a subparser.
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
        initial: Option<Token>,
    ) -> Result<(Vec<Variant>, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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

            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use zinc_lexical::IntegerLiteral as LexicalIntegerLiteral;
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::identifier::Identifier;
    use crate::tree::literal::integer::Literal as IntegerLiteral;
    use crate::tree::variant::Variant;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Variant>::new(),
            Some(Token::new(Lexeme::Eof, Location::test(1, 1))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"A = 1"#;

        let expected = Ok((
            vec![Variant::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::test(1, 5),
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 6))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"A = 1,"#;

        let expected = Ok((
            vec![Variant::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "A".to_owned()),
                IntegerLiteral::new(
                    Location::test(1, 5),
                    LexicalIntegerLiteral::new_decimal("1".to_owned()),
                ),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 7))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"A = 1, B = 2, C = 3"#;

        let expected = Ok((
            vec![
                Variant::new(
                    Location::test(1, 1),
                    Identifier::new(Location::test(1, 1), "A".to_owned()),
                    IntegerLiteral::new(
                        Location::test(1, 5),
                        LexicalIntegerLiteral::new_decimal("1".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::test(1, 8),
                    Identifier::new(Location::test(1, 8), "B".to_owned()),
                    IntegerLiteral::new(
                        Location::test(1, 12),
                        LexicalIntegerLiteral::new_decimal("2".to_owned()),
                    ),
                ),
                Variant::new(
                    Location::test(1, 15),
                    Identifier::new(Location::test(1, 15), "C".to_owned()),
                    IntegerLiteral::new(
                        Location::test(1, 19),
                        LexicalIntegerLiteral::new_decimal("3".to_owned()),
                    ),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::test(1, 20))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
