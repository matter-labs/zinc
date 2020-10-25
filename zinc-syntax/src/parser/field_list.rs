//!
//! The field list parser.
//!

use std::cell::RefCell;
use std::rc::Rc;

use zinc_lexical::Lexeme;
use zinc_lexical::Symbol;
use zinc_lexical::Token;
use zinc_lexical::TokenStream;

use crate::error::ParsingError;
use crate::parser::field::Parser as FieldParser;
use crate::tree::field::Field;

///
/// The field list parser.
///
#[derive(Default)]
pub struct Parser {
    /// The parsed fields.
    fields: Vec<Field>,
    /// The token returned from a subparser.
    next: Option<Token>,
}

impl Parser {
    ///
    /// Parses a structure field list.
    ///
    /// 'a: u8, b: field, c: (bool, u8)'
    ///
    pub fn parse(
        mut self,
        stream: Rc<RefCell<TokenStream>>,
        initial: Option<Token>,
    ) -> Result<(Vec<Field>, Option<Token>), ParsingError> {
        self.next = initial;

        loop {
            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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

            match crate::parser::take_or_next(self.next.take(), stream.clone())? {
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
    use zinc_lexical::Lexeme;
    use zinc_lexical::Location;
    use zinc_lexical::Token;
    use zinc_lexical::TokenStream;

    use super::Parser;
    use crate::tree::field::Field;
    use crate::tree::identifier::Identifier;
    use crate::tree::r#type::variant::Variant as TypeVariant;
    use crate::tree::r#type::Type;

    #[test]
    fn ok_empty() {
        let input = r#""#;

        let expected = Ok((
            Vec::<Field>::new(),
            Some(Token::new(Lexeme::Eof, Location::test(1, 1))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single() {
        let input = r#"a: u232"#;

        let expected = Ok((
            vec![Field::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "a".to_owned()),
                Type::new(Location::test(1, 4), TypeVariant::integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 8))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_single_with_comma() {
        let input = r#"a: u232,"#;

        let expected = Ok((
            vec![Field::new(
                Location::test(1, 1),
                Identifier::new(Location::test(1, 1), "a".to_owned()),
                Type::new(Location::test(1, 4), TypeVariant::integer_unsigned(232)),
            )],
            Some(Token::new(Lexeme::Eof, Location::test(1, 9))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }

    #[test]
    fn ok_multiple() {
        let input = r#"a: u232, b: i128, c: u104"#;

        let expected = Ok((
            vec![
                Field::new(
                    Location::test(1, 1),
                    Identifier::new(Location::test(1, 1), "a".to_owned()),
                    Type::new(Location::test(1, 4), TypeVariant::integer_unsigned(232)),
                ),
                Field::new(
                    Location::test(1, 10),
                    Identifier::new(Location::test(1, 10), "b".to_owned()),
                    Type::new(Location::test(1, 13), TypeVariant::integer_signed(128)),
                ),
                Field::new(
                    Location::test(1, 19),
                    Identifier::new(Location::test(1, 19), "c".to_owned()),
                    Type::new(Location::test(1, 22), TypeVariant::integer_unsigned(104)),
                ),
            ],
            Some(Token::new(Lexeme::Eof, Location::test(1, 26))),
        ));

        let result = Parser::default().parse(TokenStream::test(input).wrap(), None);

        assert_eq!(result, expected);
    }
}
